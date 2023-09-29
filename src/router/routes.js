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
