const {
    __async
} = await import(`./chunk-H3KQGMCP.js${importExpanded}`);

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
