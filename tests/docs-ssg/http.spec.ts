import { expect, Page, test } from "@playwright/test";

/// A built documentation site as readers reach it: through `rustify-docs
/// preview` at the root (`docs-root`) and under a base path
/// (`docs-subpath`), and through a plain static host that sends no headers
/// and rewrites nothing (`docs-meta`), where each page's <meta> carries the
/// whole policy. Every link on every page leads to a page that loads with
/// nothing blocked and nothing fetched from elsewhere; any address with
/// nothing behind it, at any depth, is a real 404 that still looks like the
/// site.

const STRICT =
    "default-src 'none'; script-src 'self' 'wasm-unsafe-eval'; style-src 'self'; img-src 'self' data:; font-src 'self'; connect-src 'self'; media-src 'self'; worker-src 'self'; object-src 'none'; base-uri 'none'; form-action 'none'; frame-ancestors 'none'";
const META = STRICT.replace("; frame-ancestors 'none'", "");

declare global {
    interface Window {
        __violations: string[];
    }
}

function base(): string {
    return test.info().project.name === "docs-root" ? "/" : "/rustify-ui/";
}

function origin(): string {
    return new URL(String(test.info().project.use.baseURL)).origin;
}

/// Records policy violations from the first script on.
async function watch(page: Page): Promise<{ requests: string[]; failures: string[] }> {
    await page.addInitScript(() => {
        window.__violations = [];
        document.addEventListener("securitypolicyviolation", (event) => {
            window.__violations.push(`${event.violatedDirective} ${event.blockedURI}`);
        });
    });
    const seen = { requests: [] as string[], failures: [] as string[] };
    page.on("request", (request) => seen.requests.push(request.url()));
    page.on("requestfailed", (request) => seen.failures.push(`${request.url()} ${request.failure()?.errorText}`));
    page.on("response", (response) => {
        if (response.status() >= 400) {
            seen.failures.push(`${response.url()} ${response.status()}`);
        }
    });
    return seen;
}

test("every page links only to pages that load whole, under the base, from this origin", async ({ page }) => {
    const seen = await watch(page);
    const start = `${origin()}${base()}`;
    const queue = [start];
    const visited = new Set<string>();
    while (queue.length > 0) {
        const url = queue.shift()!;
        if (visited.has(url)) {
            continue;
        }
        visited.add(url);
        const response = await page.goto(url);
        expect(response?.status(), url).toBe(200);
        expect(await page.evaluate(() => window.__violations), url).toEqual([]);
        expect(await page.locator("h1").count(), `${url}: one h1`).toBe(1);
        // Every address the page names is under the base, and external links
        // open no window with a handle back to this one.
        const addresses = await page.evaluate(() =>
            [...document.querySelectorAll("[href], [src]")].map((element) => ({
                tag: element.tagName,
                value: element.getAttribute("href") ?? element.getAttribute("src") ?? "",
                rel: element.getAttribute("rel") ?? "",
            })),
        );
        for (const { tag, value, rel } of addresses) {
            if (value.startsWith("/")) {
                expect(value.startsWith(base()), `${url}: ${value}`).toBe(true);
            } else if (/^https?:/.test(value) && tag === "A") {
                expect(rel.split(/\s+/), `${url}: ${value}`).toContain("noopener");
            }
        }
        const links = await page.evaluate(() =>
            [...document.querySelectorAll("a[href]")].map((a) => (a as HTMLAnchorElement).href),
        );
        for (const link of links) {
            const target = new URL(link);
            if (target.origin === origin() && target.pathname.startsWith(base())) {
                target.hash = "";
                queue.push(target.href);
            }
        }
    }
    expect(visited.size, [...visited].join("\n")).toBe(4);
    expect(seen.failures).toEqual([]);
    for (const request of seen.requests) {
        expect(new URL(request).origin, request).toBe(origin());
    }
});

test("an anchor link lands on its heading, Chinese ones included", async ({ page }) => {
    await page.goto(`${base()}guide/`);
    await page.locator("article a", { hasText: "reference" }).click();
    await expect(page).toHaveURL(new RegExp(`${base()}reference/api/#%E6%95%B0%E6%8D%AE-api$`));
    await expect(page.locator("h2#数据-api")).toBeInViewport();
});

test("an address with nothing behind it is a real 404 that looks like the site, at any depth", async ({ page }) => {
    const seen = await watch(page);
    for (const missing of ["nope/", "a/b/c/d/missing/", "guide/setup/nope.html", "assets/site.0000000000.css"]) {
        const response = await page.goto(`${base()}${missing}`);
        expect(response?.status(), missing).toBe(404);
        await expect(page.locator("h1")).toHaveText("Page not found");
        // The stylesheet loaded: the body has the theme's background.
        const background = await page.evaluate(() => getComputedStyle(document.body).backgroundColor);
        expect(background, missing).not.toBe("rgba(0, 0, 0, 0)");
        expect(await page.evaluate(() => window.__violations)).toEqual([]);
    }
    expect(seen.failures.every((failure) => failure.endsWith(" 404"))).toBe(true);
    await page.getByRole("link", { name: "Back to the home page" }).click();
    await expect(page).toHaveURL(`${origin()}${base()}`);
});

test("the policy arrives as a header where the host can send one, and in every page", async ({ page, request }) => {
    const response = await request.get(base());
    const header = response.headers()["content-security-policy"];
    if (test.info().project.name === "docs-meta") {
        expect(header).toBeUndefined();
    } else {
        expect(header).toBe(STRICT);
    }
    await page.goto(base());
    await expect(page.locator('meta[http-equiv="Content-Security-Policy"]')).toHaveAttribute("content", META);
    // The policy is in force: an inline script does not run.
    const ran = await page.evaluate(() => {
        const script = document.createElement("script");
        script.textContent = "window.__inline = true";
        document.head.append(script);
        return (window as unknown as { __inline?: boolean }).__inline === true;
    });
    expect(ran).toBe(false);
});

test("a directory without its slash is redirected, and nothing outside the base is served", async ({ request }) => {
    const redirect = await request.get(`${base()}guide`, { maxRedirects: 0 });
    expect(redirect.status()).toBe(301);
    expect(redirect.headers()["location"]).toBe(`${base()}guide/`);
    if (base() !== "/") {
        const outside = await request.get("/guide/");
        expect(outside.status()).toBe(404);
    }
});
