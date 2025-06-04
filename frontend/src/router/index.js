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
      { path: '', component: MainPage },
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
  const token = localStorage.getItem('token')
  if (to.path != '/login' && !token) {
    next('/login')
  } else if ( to.path == '/admin' &&  token != 'hardcoded-admin-token') {
    next('/login')
    alert("No permission.")
  } else {
    next()
  }
})

export default router
