import Navbar from "@/components/Navbar";
import { Box, Container } from "@mui/material";
import { Outlet } from "react-router-dom";

export function Default() {
    return (
        <>
            <Navbar />
            <Box
                sx={{
                    position: "relative",
                    height: "calc(100vh - 64px)",
                }}
            >
                <Outlet />
            </Box>
        </>
    );
}
