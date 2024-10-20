import { alovaInstance } from "@/utils/alova";
import { LoadingButton } from "@mui/lab";
import { Avatar, Box, Button, Card, Paper, Typography } from "@mui/material";
import { useRequest } from "alova/client";
import { useEffect, useState } from "react";

export default function CTFShow() {
    const { loading, data, send } = useRequest(alovaInstance.Get("/ctfshow"), {
        immediate: false,
    });

    const { data: data2, send: send2 } = useRequest(
        alovaInstance.Get("/ctfshow/status")
    );

    const [status, setStatus] = useState<any>();
    const [timestamp, setTimestamp] = useState<number>(Date.now() / 1000);
    const [minute, setMinute] = useState<number>(0);
    const [second, setSecond] = useState<number>(0);

    useEffect(() => {
        setStatus(data2?.data);
    }, [data2]);

    useEffect(() => {
        const interval = setInterval(() => {
            setTimestamp(Date.now() / 1000);
        }, 1000);
        return () => clearInterval(interval);
    }, []);

    useEffect(() => {
        if (status?.obtained_at) {
            let duration = timestamp - status?.obtained_at;
            setMinute(duration / 60);
            setSecond(duration % 60);
        }
    }, [timestamp]);

    return (
        <Paper
            sx={{
                position: "absolute",
                top: "50%",
                left: "50%",
                transform: "translate(-50%, -50%)",
                p: 5,
                display: "flex",
                flexDirection: "column",
                gap: 2,
            }}
        >
            <h1>CTFShow 登录器</h1>
            <p>
                每个账号在 24 小时内可申请 2 次登录 Session，在上一人申请后 3
                小时内不可申请。
            </p>
            <LoadingButton
                loading={loading}
                variant="contained"
                onClick={() => {
                    send();
                }}
            >
                申请
            </LoadingButton>
            <Box
                sx={{
                    display: "flex",
                    justifyContent: "space-between",
                    alignItems: "center",
                }}
            >
                <Box
                    sx={{
                        display: "flex",
                        alignItems: "center",
                        gap: 2,
                    }}
                >
                    <Avatar
                        alt={status?.user?.login}
                        src={status?.user?.avatar_url}
                    />
                    <Typography>{status?.user?.login}</Typography>
                </Box>
                <Typography>
                    已使用：{Math.floor(minute)}分{Math.floor(second)}秒
                </Typography>
            </Box>
            <Box>
                <pre>{JSON.stringify(data, null, 4)}</pre>
            </Box>
        </Paper>
    );
}
