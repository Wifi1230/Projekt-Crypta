import { createRouter, createWebHistory } from 'vue-router';
import Home from './views/Home.vue';
import Login from './views/Login.vue';
import Rejestracja from './views/Rejestracja.vue';
import UploadPost from './views/UploadPost.vue';
import Voting from './views/Voting.vue'; 

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
  },
  {
    path: '/upload',
    name: 'UploadPost',
    component: UploadPost
  },
  {
    path: '/voting',  // Nowa trasa
    name: 'Voting',
    component: Voting
  }
];

const router = createRouter({
  history: createWebHistory(),
  routes
});

export default router;