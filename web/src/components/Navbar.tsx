import { useAuthStore } from "@/stores/auth";
import {
    AppBar,
    Container,
    Toolbar,
    Typography,
    Box,
    IconButton,
    Menu,
    MenuItem,
    Avatar,
    ListItemIcon,
    ListItemText,
    Button,
} from "@mui/material";
import { useState } from "react";
import LogoutIcon from "@mui/icons-material/Logout";
import VpnLockIcon from "@mui/icons-material/VpnLock";
import GitHubIcon from "@mui/icons-material/GitHub";
import { alovaInstance } from "@/utils/alova";
import { useRequest } from "alova/client";
import { LoadingButton } from "@mui/lab";
import { useNavigate } from "react-router-dom";

export default function Navbar() {
    const authStore = useAuthStore();
    const navigate = useNavigate();

    const github = () => {
        const method = alovaInstance.Get<{ data: string }>("/github");
        method.meta = {
            authRole: null,
        };
        return method;
    };

    const { send } = useRequest(github(), { immediate: false });

    const [anchorElUser, setAnchorElUser] = useState<HTMLElement>();

    const handleClickLogin = () => {
        send().then((res) => {
            window.location.href = `https://github.com/login/oauth/authorize?client_id=${res.data}&scope=user:email`;
        });
    };

    const handleClickAvatar = (event: React.MouseEvent<HTMLElement>) => {
        setAnchorElUser(event.currentTarget);
    };

    const handleCloseUserMenu = () => {
        setAnchorElUser(undefined);
    };

    const handleLogout = () => {
        navigate("/");
        authStore.clear();
        handleCloseUserMenu();
    };

    return (
        <AppBar position="static">
            <Container maxWidth="xl">
                <Toolbar
                    disableGutters
                    sx={{
                        display: "flex",
                        justifyContent: "space-between",
                        gap: 5,
                    }}
                >
                    <Button
                        variant="contained"
                        disableElevation
                        sx={{
                            display: "flex",
                            gap: 2,
                            alignItems: "center",
                        }}
                        onClick={() => navigate("/")}
                    >
                        <VpnLockIcon />
                        <Typography variant="h6" noWrap component="a">
                            P1anet
                        </Typography>
                    </Button>
                    <Box
                        sx={{
                            flex: 1,
                            display: "flex",
                            justifyContent: "flex-start",
                        }}
                        onClick={() => navigate("/ctfshow")}
                    >
                        <Button sx={{ color: "white", display: "block" }}>
                            CTFShow 登录器
                        </Button>
                    </Box>
                    <Box>
                        {authStore?.user ? (
                            <>
                                <IconButton
                                    onClick={handleClickAvatar}
                                    sx={{ p: 0 }}
                                >
                                    <Avatar
                                        alt={authStore?.user?.login}
                                        src={authStore?.user?.avatar_url}
                                    />
                                </IconButton>
                                <Menu
                                    sx={{ mt: "60px" }}
                                    anchorEl={anchorElUser}
                                    anchorOrigin={{
                                        vertical: "top",
                                        horizontal: "right",
                                    }}
                                    keepMounted
                                    transformOrigin={{
                                        vertical: "top",
                                        horizontal: "right",
                                    }}
                                    open={Boolean(anchorElUser)}
                                    onClose={handleCloseUserMenu}
                                >
                                    <MenuItem onClick={handleLogout}>
                                        <ListItemIcon>
                                            <LogoutIcon />
                                        </ListItemIcon>
                                        <ListItemText>登出</ListItemText>
                                    </MenuItem>
                                </Menu>
                            </>
                        ) : (
                            <LoadingButton
                                loading={authStore.loading}
                                size="large"
                                loadingPosition="start"
                                variant="contained"
                                disableElevation
                                startIcon={<GitHubIcon />}
                                sx={{
                                    color: "white",
                                }}
                                onClick={handleClickLogin}
                            >
                                通过 GitHub 登录
                            </LoadingButton>
                        )}
                    </Box>
                </Toolbar>
            </Container>
        </AppBar>
    );
}
