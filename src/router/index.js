import Vue from 'vue'
import VueRouter from 'vue-router'

Vue.use(VueRouter)

const routes = [
  {
    path: '/',
    name: 'Dashboard',
    component: function () {
      return import(/* webpackChunkName: "about" */ '../views/Dashboard.vue')
    }
  },
  {
    path: '/connect',
    name: 'Connect',
    component: function () {
      return import(/* webpackChunkName: "about" */ '../views/Connect.vue')
    }
  }
]

const router = new VueRouter({
  mode: 'history',
  routes
})

export default router
