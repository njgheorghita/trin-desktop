import Configure from '@/pages/Configure.vue'
import Home from '@/pages/Home.vue'
import JsonRpc from '@/pages/JsonRpc.vue'
import Account from '@/pages/Account.vue'
import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      component: Home,
      name: 'home',
      meta: { keepAlive: true }
    },
    {
      path: '/configure',
      component: Configure,
      name: 'configure'
    },
    {
      path: '/jsonrpc',
      component: JsonRpc,
      name: 'jsonrpc'
    },
    {
      path: '/account',
      component: Account,
      name: 'account'
    }
  ]
})

export default router
