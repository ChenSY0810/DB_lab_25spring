import { createRouter, createWebHistory } from 'vue-router'
import AppLayout from '@/AppLayout.vue'
import LoginPage from '@/views/LoginPage.vue'
import MainPage from '@/views/MainPage.vue'
import AdminPage from '@/views/AdminPage.vue'
import ProfilePage from '@/views/ProfilePage.vue'

const routes = [
  {
    path: '/',
    component: AppLayout,
    children: [
      { path: 'user-profile', component: ProfilePage },
      { path: 'main', component: MainPage },
      { path: 'admin', component: AdminPage},
    ]
  },
  { path: '/login', component: LoginPage }
]


const router = createRouter({
  history: createWebHistory(),
  routes
})

// 路由守卫
router.beforeEach((to, from, next) => {
  const pri = Number(localStorage.getItem('privilege'))
  if (to.path != '/login' && !pri) {
    next('/login')
  } else if ( to.path == '/admin' &&  pri != 2) {
    alert("No permission.")
    if (pri === 1) {
      next('/main')
    } else {
      next('/login')
    }
  } else {
    next()
  }
})

export default router
