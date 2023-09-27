const routes = [
  { path: "/", name: "Home", component: () => import("../pages/Index.vue") },
  {
    path: "/about",
    name: "About",
    component: () => import("../pages/About.vue"),
  },

  // Always leave this as last one,
  // but you can also remove it
  {
    path: "/:catchAll(.*)*",
    component: () => import("../pages/Error404.vue"),
  },
];

export default routes;
