<template>
    <div class="bg-slate-900  h-screen flex flex-col items-center justify-center">
<!-- Nagłówek Rejestracja -->
      <h1 class="text-3xl font-semibold text-white mt-8">Rejestracja</h1>
      
      <!-- Formularz rejestracji -->
      <div class="max-w-md w-full mx-auto p-8 bg-gray-700 border border-gray-900 rounded-lg shadow-lg">
        <form @submit.prevent="register" class="space-y-4">
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
          <button type="submit"
            class="w-full px-4 py-2 bg-gray-900 text-white rounded-md hover:bg-gray-800 focus:outline-none focus:ring focus:ring-gray-400">
            Register
          </button>
        </form>
  
        <!-- Link do logowania -->
        <div class="text-sm text-gray-400 mt-2 text-center">
          Masz już konto? <router-link to="/login" class="underline hover:text-gray-300">Zaloguj się</router-link>
        </div>
      </div>
    </div>
  </template>
  
  <script setup>
  import { ref } from 'vue';
  import { useRouter } from 'vue-router';
  
  const router = useRouter();
  
  const username = ref('');
  const email = ref('');
  const password = ref('');
  
  const register = () => {
    // Sprawdź, czy użytkownik już istnieje w localStorage
    if (localStorage.getItem(username.value)) {
      alert('Użytkownik już istnieje');
      return;
    }
  
    // Zapisz dane do localStorage
    localStorage.setItem(username.value, JSON.stringify({ email: email.value, password: password.value }));
  
    // Przekierowanie na stronę logowania po zarejestrowaniu
    router.push('/login');
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
  