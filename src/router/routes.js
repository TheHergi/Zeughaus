const routes = [
  {
    path: '/',
    name: 'Home',
    component: () => import('../views/HomeView.vue')
  },
  {
    path: '/character',
    component: () => import('../views/CharacterView.vue'),
    children: [
      {
        path: '/character/general',
        name: 'Character',
        component: () => import('../views/CharacterViewGeneral.vue')
      },
      {
        path: '/character/attributes',
        name: 'Spielwerte',
        component: () => import('../views/CharacterViewAttributes.vue')
      },
      {
        name: 'Fähigkeiten',
        path: '/character/skills',
        children: [
          {
            path: '/character/skills/basic',
            name: 'Grundfähigkeiten',
            component: () => import('../views/CharacterViewSkillsBasic.vue')
          },
          {
            path: '/character/skills/advanced',
            name: 'Ausbaufähigkeiten',
            component: () => import('../views/CharacterViewSkillsAdvanced.vue')
          }
        ]
      },
      {
        name: 'Talente',
        path: '/character/talents',
        component: () => import('../views/CharacterViewTalents.vue')
      },
      {
        name: 'Zauber',
        path: '/character/spells',
        component: () => import('../views/CharacterViewSpells.vue')
      },
      {
        name: 'Gebete',
        path: '/character/prayer',
        component: () => import('../views/CharacterViewPrayers.vue')
      },
      {
        name: 'Ausrüstung',
        path: '/character/trappings',
        component: () => import('../views/CharacterViewTrappings.vue')
      }
    ]
  },
  {
    path: '/wiki',
    name: 'Wiki',
    component: () => import('../views/WikiView.vue')
  },
  {
    path: '/about',
    name: 'About',
    component: () => import('../views/AboutView.vue')
  },

  // Always leave this as last one,
  // but you can also remove it
  {
    path: '/:catchAll(.*)*',
    component: () => import('../views/Error404.vue')
  }

]

export default routes
