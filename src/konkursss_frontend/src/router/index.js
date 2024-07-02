import { createRouter, createWebHistory } from 'vue-router';
import Home from '../views/Home.vue';
import Login from '../views/Login.vue';
import Rejestracja from '../views/Rejestracja.vue';

const routes = [
  {
    path: '/',
    name: 'Home',
    component: Home
  },
  {
    path: '/login',
    name: 'Login',
    component: Login
  },
  {
    path: '/rejestracja',
    name: 'Rejestracja',
    component: Rejestracja
  }
];

const router = createRouter({
  history: createWebHistory(),
  routes
});

export default router;