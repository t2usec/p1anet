import { createBrowserRouter } from "react-router-dom";

export const router = createBrowserRouter([
    {
        path: "/",
        lazy: async () => {
            let { Default } = await import("@/layouts/Default");
            return { Component: Default };
        },
        children: [
            {
                index: true,
                lazy: async () => {
                    let Page = await import("@/pages");
                    return { Component: Page.default };
                },
            },
            {
                path: "callback",
                lazy: async () => {
                    let Page = await import("@/pages/callback");
                    return { Component: Page.default };
                },
            },
            {
                path: "ctfshow",
                lazy: async () => {
                    let Page = await import("@/pages/ctfshow");
                    return { Component: Page.default };
                },
            },
        ],
    },
]);
