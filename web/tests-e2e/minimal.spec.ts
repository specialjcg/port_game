import { test, expect } from '@playwright/test';

// Configuration du test avec retry et timeouts appropriés
test.describe('Port Game - Tests fondamentaux', () => {
  test.setTimeout(30000);

  test.beforeEach(async ({ page }) => {
    // Attente explicite que le serveur WASM soit prêt
    await page.goto('http://localhost:5173');
    // Attendre que le jeu soit réellement chargé (état initial visible)
    await page.waitForSelector('.port-header', { state: 'visible', timeout: 10000 });
  });

  test('vérification de l\'initialisation du jeu', async ({ page }) => {
    // Test des éléments fondamentaux du jeu
    // 1. Interface de base
    await expect(page.locator('text=PORT TERMINAL MANAGER')).toBeVisible();

    // 2. Éléments de jeu initiaux
    await expect(page.locator('.berth')).toHaveCount(2); // 2 berths par défaut
    await expect(page.locator('.crane')).toHaveCount(2); // 2 grues par défaut
    await expect(page.locator('.ship')).toBeVisible(); // Au moins un navire présent

    // 3. Contrôles de jeu
    await expect(page.getByText(/End Turn|Finir le tour/i)).toBeEnabled();

    // 4. Score initial
    const score = await page.locator('.port-score').innerText();
    expect(score).toContain('Score: 0');
  });

  test('interaction de base - cycle de jeu', async ({ page }) => {
    // 1. Vérifier qu'on peut finir un tour
    await page.getByText(/End Turn|Finir le tour/i).click();
    await page.waitForTimeout(500); // Attendre la mise à jour du jeu

    // 2. Vérifier que le jeu continue
    await expect(page.getByText(/End Turn|Finir le tour/i)).toBeEnabled();

    // 3. Vérifier que l'état du jeu est cohérent
    const portSection = await page.locator('.port').first();
    await expect(portSection).toBeVisible();
  });
});
