<template>
  <div class="bg-slate-900 h-screen flex flex-col items-center justify-center">
    <!-- Nagłówek Rejestracja -->
    <h1 class="text-3xl font-semibold text-white mt-8">Register</h1>
    
    <!-- Formularz rejestracji -->
    <div class="max-w-md w-full mx-auto p-8 bg-gray-700 border border-gray-900 rounded-lg shadow-lg">
      <form @submit.prevent="sendDatatoBack" class="space-y-4">
        <div>
          <label for="username" class="block text-sm font-medium text-white">Username</label>
          <input v-model="username" type="text" id="username" name="username" placeholder="Enter your username"
            class="mt-1 block w-full px-3 py-2 rounded-md bg-gray-800 text-white focus:outline-none focus:ring focus:ring-gray-400"
            required>
        </div>
        <div>
          <label for="email" class="block text-sm font-medium text-white">Email</label>
          <input v-model="email" type="email" id="email" name="email" placeholder="Enter your email"
            class="mt-1 block w-full px-3 py-2 rounded-md bg-gray-800 text-white focus:outline-none focus:ring focus:ring-gray-400"
            required>
        </div>
        <div>
          <label for="password" class="block text-sm font-medium text-white">Password</label>
          <input v-model="password" type="password" id="password" name="password" placeholder="Enter your password"
            class="mt-1 block w-full px-3 py-2 rounded-md bg-gray-800 text-white focus:outline-none focus:ring focus:ring-gray-400"
            required>
        </div>
        <div class="flex items-center">
        <input type="checkbox" id="terms" name="terms" required class="h-4 w-4 text-slate-900 border-gray-600 rounded focus:ring-slate-900 checked:bg-slate-900 checked:border-slate-900 focus:ring-2">
        <label for="terms" class="ml-2 text-sm text-white">I agree to the terms and conditions</label>
         </div>
        <button type="submit"
          class="w-full px-4 py-2 bg-gray-900 text-white rounded-md hover:bg-gray-800 focus:outline-none focus:ring focus:ring-gray-400">
          Register
        </button>
      </form>

      <!-- Wiadomość o błędzie -->
      <div v-if="errorMessage" class="mt-4 text-red-500 text-center">
        {{ errorMessage }}
      </div>
  
      <!-- Link do logowania -->
      <div class="text-sm text-gray-400 mt-2 text-center">
        You have account? <router-link to="/login" class="underline hover:text-gray-300">Log in</router-link>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import { konkursss_backend } from 'declarations/konkursss_backend/index';

const router = useRouter();

const username = ref('');
const email = ref('');
const password = ref('');
const errorMessage = ref('');

const sendDatatoBack = async () => {
  try {
    const userData = {
      username: username.value,
      email: email.value,
      password: password.value
    };
    const response = await konkursss_backend.add_account(userData);

    if (response.Ok !== undefined) {
      router.push('/login');
    } else if (response.Err !== undefined) {
      errorMessage.value = response.Err;
    }
  } catch (error) {
    console.error('Error sending data to backend:', error);
    errorMessage.value = 'Unexpected error occurred. Please try again.';
  }
};
</script>

<style scoped>
/* Dodatkowy styl dla przycisku Register */
button {
  transition: background-color 0.3s ease;
}

button:hover {
  background-color: #4a5568; /* Ciemniejszy odcień szarości */
}

.bg-gray-900 {
  background-color: #2d3748; /* Kolor tła jak w sekcji Home */
}
</style>
