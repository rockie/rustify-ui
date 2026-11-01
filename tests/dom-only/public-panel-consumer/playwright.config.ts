import { defineConfig } from '@playwright/test';

export default defineConfig({
    testDir: '.',
    testMatch: 'public-panel.spec.ts',
    outputDir: '../../../test-results/public-panel',
    workers: 1,
    use: { baseURL: 'http://127.0.0.1:4206', viewport: { width: 1280, height: 900 }, trace: 'retain-on-failure' },
    projects: [{ name: 'public-panel', use: { browserName: 'chromium' } }],
    webServer: {
        command: 'mbx xtask serve --dir target/dom/public-panel-consumer --port 4206',
        cwd: '../../..',
        url: 'http://127.0.0.1:4206',
        reuseExistingServer: false,
        timeout: 120_000,
    },
});
