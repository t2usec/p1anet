import { createAlova } from "alova";
import adapterFetch from "alova/fetch";
import ReactHook from "alova/react";
import { createClientTokenAuthentication } from "alova/client";
import { useAuthStore } from "@/stores/auth";
import { jwtDecode } from "jwt-decode";

const { onAuthRequired, onResponseRefreshToken } =
    createClientTokenAuthentication({
        assignToken: (method) => {
            method.config.headers["Authorization"] =
                `Bearer ${useAuthStore.getState().token}`;
        },
        refreshToken: {
            isExpired: (_method) => {
                try {
                    const decoded = jwtDecode(
                        String(useAuthStore.getState().token)
                    );

                    return (decoded?.exp || 0) < Date.now() / 1000;
                } catch {
                    return true;
                }
            },
            handler: async (_method) => {
                location.href = "/";
            },
        },
    });

export const alovaInstance = createAlova({
    baseURL: "/api",
    requestAdapter: adapterFetch(),
    timeout: 0,
    shareRequest: true,
    statesHook: ReactHook,
    responded: onResponseRefreshToken({
        onSuccess: (response, _method) => {
            return response.json();
        },
    }),
    beforeRequest: onAuthRequired((_method) => {}),
});
