import {
    __async
} from "./chunk-H3KQGMCP.js";

// src/app/pages/auth/index.ts
var routes = [
    {
        path: "login",
        title: "Login",
        loadComponent: () => __async(void 0, null, function* () {
            return (yield import("./chunk-ZDWTUTB7.js")).LoginComponent;
        })
    }
];
export {
    routes
};
