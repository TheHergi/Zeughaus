const routes = [
  {
    path: "/",
    component: () => import("../layouts/MainLayout.vue"),
    children: [
      {
        path: "/",
        name: "Home",
        component: () => import("../views/Index.vue"),
      },
      {
        path: "/character",
        name: "Character",
        component: () => import("../views/Character.vue"),
      },
      {
        path: "/wiki",
        name: "Wiki",
        component: () => import("../views/Wiki.vue"),
      },
      {
        path: "/about",
        name: "About",
        component: () => import("../views/About.vue"),
      },
    ],
  },

  // Always leave this as last one,
  // but you can also remove it
  {
    path: "/:catchAll(.*)*",
    component: () => import("../views/Error404.vue"),
  },
];

export default routes;
