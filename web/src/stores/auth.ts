import { User } from "@/models/user";
import { create } from "zustand";
import { createJSONStorage, persist } from "zustand/middleware";

interface AuthState {
    user?: User;
    token?: string;
    loading?: boolean;
    setLoading: (loading?: boolean) => void;
    setUser: (user?: User) => void;
    setToken: (token?: string) => void;
    clear: () => void;
}

export const useAuthStore = create<AuthState>()(
    persist(
        (set, get) => ({
            setUser: (user?: User) => set({ user }),
            setToken: (token?: string) => set({ token }),
            clear: () => set({ user: undefined, token: undefined }),
            setLoading: (loading?: boolean) => set({ loading }),
        }),
        {
            name: "auth",
            storage: createJSONStorage(() => localStorage),
        }
    )
);
