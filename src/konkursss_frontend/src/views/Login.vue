<template>
  <div class="bg-slate-900 h-screen flex flex-col items-center justify-center">
    <h1 class="text-3xl font-semibold text-white mt-8">Log in</h1>
    <div class="max-w-md w-full mx-auto p-8 bg-gray-700 border border-gray-900 rounded-lg shadow-lg mt-8">
      <form @submit.prevent="login" class="space-y-4">
        <div>
          <label for="username" class="block text-sm font-medium text-white">Username</label>
          <input v-model="username" type="text" id="username" name="username" placeholder="Enter your username"
            class="mt-1 block w-full px-3 py-2 rounded-md bg-gray-800 text-white focus:outline-none focus:ring focus:ring-gray-400" required>
        </div>
        <div>
          <label for="password" class="block text-sm font-medium text-white">Password</label>
          <input v-model="password" type="password" id="password" name="password" placeholder="Enter your password"
            class="mt-1 block w-full px-3 py-2 rounded-md bg-gray-800 text-white focus:outline-none focus:ring focus:ring-gray-400" required>
        </div>
        <button type="submit" class="w-full px-4 py-2 bg-gray-900 text-white rounded-md hover:bg-gray-800 focus:outline-none focus:ring focus:ring-gray-400">
          Log in
        </button>
      </form>

      <div v-if="error" class="text-sm text-red-500 mt-2 text-center">{{ error }}</div>
      <div class="text-sm text-gray-400 mt-2 text-center">
        Nie masz konta? <router-link to="/rejestracja" class="underline hover:text-gray-300">Zarejestruj się</router-link>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import { konkursss_backend } from 'declarations/konkursss_backend/index';
import { userStore } from '../store'; // importuj globalny stan użytkownika

const router = useRouter();
const username = ref('');
const password = ref('');
const error = ref('');

const login = async () => {
  try {
    const accounts = await konkursss_backend.get_all_accounts();
    const user = accounts.find(account => account.username === username.value && account.password === password.value);

    if (user) {
      userStore.setUsername(user.username); // ustaw globalny stan użytkownika
      router.push('/');
    } else {
      error.value = 'Invalid username or password';
    }
  } catch (err) {
    console.error('Error logging in:', err);
    error.value = 'Error logging in. Please try again.';
  }
};
</script>

<style scoped>
button {
  transition: background-color 0.3s ease;
}

button:hover {
  background-color: #4a5568;
}

.bg-gray-900 {
  background-color: #2d3748;
}
</style>
