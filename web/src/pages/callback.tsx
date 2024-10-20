import { User } from "@/models/user";
import { useAuthStore } from "@/stores/auth";
import { alovaInstance } from "@/utils/alova";
import { Container } from "@mui/material";
import { useRequest } from "alova/client";
import { useEffect } from "react";
import { useLocation, useNavigate } from "react-router-dom";

export default function Callback() {
    const location = useLocation();
    const navigate = useNavigate();
    const queryParams = new URLSearchParams(location.search);
    const code = queryParams.get("code");

    const authStore = useAuthStore();

    const cb = () => {
        const method = alovaInstance.Get<{ data: User; token: string }>(
            "/auth/callback",
            {
                params: {
                    code: code,
                },
            }
        );
        method.meta = {
            authRole: null,
        };
        return method;
    };

    const { loading, data, send } = useRequest(cb(), {
        immediate: false,
    });

    useEffect(() => {
        if (code) {
            authStore.setLoading(true);
            send();
        }
    }, [code]);

    useEffect(() => {
        if (data) {
            authStore.setLoading(false);
            authStore.setToken(data?.token);
            authStore.setUser(data?.data);
            navigate("/");
        }
    }, [data]);

    return (
        <>
            <Container>
                <h1>Callback {code}</h1>
                {loading && <p>Loading...</p>}
                {data && (
                    <div>
                        <p>Username: {data?.data?.login}</p>
                        <p>Token: {data?.token}</p>
                    </div>
                )}
            </Container>
        </>
    );
}
