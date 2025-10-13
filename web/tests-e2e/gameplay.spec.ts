import { test, expect } from '@playwright/test';

test.describe('Port Game - Tests de gameplay complet', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('http://localhost:5173');
    // Attendre que le jeu soit chargé
    await page.waitForSelector('.port-header');
  });

  test('démarrage du jeu avec 3 navires', async ({ page }) => {
    // Vérifier qu'il y a exactement 3 navires au départ
    await expect(page.locator('.waiting-ships .ship')).toHaveCount(3);
  });

  test('cycle complet de déchargement d\'un navire', async ({ page }) => {
    // Attendre qu'un navire et un berth soient disponibles
    const shipSelector = '.waiting-ships .ship';
    const freeBerthSelector = '.berth.free';

    await page.waitForSelector(shipSelector);
    await page.waitForSelector(freeBerthSelector);

    // Faire glisser le navire vers le berth
    await page.dragAndDrop(shipSelector, freeBerthSelector);

    // Vérifier que le berth est occupé
    await expect(page.locator('.berth.occupied')).toBeVisible();

    // Assigner une grue
    await page.click('text=Crane #0');
    await page.click('.berth.occupied .berth-ship');

    // Attendre que les conteneurs soient traités
    await page.click('text=End Turn');

    // Vérifier que les conteneurs sont en cours de traitement
    const containerInfo = await page.locator('.container-info').first();
    const initialContainers = await containerInfo.innerText();
    expect(initialContainers).toContain('/'); // format: "X / Y"
  });

  test('nouveaux navires arrivent tous les 3 tours', async ({ page }) => {
    const countShips = async () => {
      return await page.locator('.waiting-ships .ship').count();
    };

    const initialShips = await countShips();

    // Jouer 3 tours
    for(let i = 0; i < 3; i++) {
      await page.click('text=End Turn');
      await page.waitForTimeout(300);
    }

    // Vérifier que 2 nouveaux navires sont arrivés
    const shipsAfterThreeTurns = await countShips();
    expect(shipsAfterThreeTurns).toBe(initialShips + 2);
  });

  test('fin de partie après exactement 10 tours', async ({ page }) => {
    // Jouer les 10 tours
    for(let i = 0; i < 10; i++) {
      await page.click('text=End Turn');
      await page.waitForTimeout(300);
    }

    // Vérifier que le jeu est terminé
    await expect(page.getByText(/Game Over|Partie terminée/i)).toBeVisible();

    // Vérifier qu'un gagnant est déclaré
    await expect(page.getByText(/Winner|Gagnant/i)).toBeVisible();

    // Vérifier qu'on ne peut plus jouer
    await expect(page.getByText(/End Turn|Finir le tour/i)).not.toBeEnabled();
  });

  test('événements aléatoires affectent l\'efficacité des grues', async ({ page }) => {
    // Observer l'efficacité initiale des grues
    await page.click('text=End Turn');

    // Jouer jusqu'à ce qu'un événement apparaisse
    for(let i = 0; i < 5; i++) {
      const hasEvent = await page.getByText('⚠️').isVisible();
      if(hasEvent) {
        // Vérifier si l'événement mentionne l'efficacité des grues
        const eventText = await page.locator('.event-description').innerText();
        expect(eventText).toMatch(/efficiency|efficacité|crane|grue/i);
        break;
      }
      await page.click('text=End Turn');
      await page.waitForTimeout(300);
    }
  });
});
