import { isPublicSitePathForDev } from "../src/utils/publicSitePath";
/**
 * 开发模式下：访问公开站路径时 302 到后端（VITE_SERVER），避免 admin SPA 404。
 */
export function devPublicRedirectPlugin(serverUrl, localesRaw) {
    const target = serverUrl.replace(/\/$/, "");
    return {
        name: "dev-public-site-redirect",
        apply: "serve",
        configureServer(server) {
            server.middlewares.use((req, res, next) => {
                const rawUrl = req.url ?? "/";
                const pathname = rawUrl.split("?")[0] || "/";
                if (!isPublicSitePathForDev(pathname, localesRaw)) {
                    next();
                    return;
                }
                res.statusCode = 302;
                res.setHeader("Location", `${target}${rawUrl}`);
                res.end();
            });
        },
    };
}
