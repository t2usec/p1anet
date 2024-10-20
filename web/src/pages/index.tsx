import { Button, Paper, Typography } from "@mui/material";

export default function index() {
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
            <h1>密码王倾情巨献</h1>
            <Typography
                sx={{
                    color: "white",
                }}
            >
                实际上是 ElaBosak233
            </Typography>
        </Paper>
    );
}
