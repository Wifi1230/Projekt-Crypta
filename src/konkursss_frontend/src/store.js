import { reactive } from 'vue';

export const userStore = reactive({
  username: localStorage.getItem('userId') || '',
  
  setUsername(name) {
    this.username = name;
    if (name) {
      localStorage.setItem('userId', name);
    } else {
      localStorage.removeItem('userId');
    }
  }
});