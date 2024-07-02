<template>
  <div>
      <header class="bg-gray-700 w-full h-1/5 fixed top-0 z-50">
      <!-- Górna część nagłówka z logo -->
      <div class="h-3/4 flex justify-center items-center">
        <router-link to="/" class="h-full aspect-square">
          <img src="/logo2.png" alt="CRYPTA" class="h-full aspect-square"/>
        </router-link>
      </div>
      
      <!-- Dolna część nagłówka z sekcjami -->
      <div class="h-1/4 grid grid-cols-3 items-center">
        <!-- Pole wyszukiwania kryptowalut -->
        <div>
          <input
            id="crypto_name"
            v-model="searchQuery"
            @input="filterCryptocurrencies"
            @click="toggleCryptocurrencies"
            placeholder="Crypto name"
            class="text-center placeholder-text-center placeholder-white text-white bg-transparent w-full px-4 py-2 border-none focus:outline-none">
          <button class="aspect-square w-fit h-full"></button>
        </div>
        <!-- Sekcja "Voting" -->
        <div class="text-white text-center">
          <p>Voting</p>
        </div>
        
        <!-- Sekcja "Log in" -->
        <div class="text-white text-center">
          <router-link to="/login">Log in</router-link>
        </div>
      </div>
        <!-- Lista filtrowanych kryptowalut z przewijaniem -->
        <div v-if="showCryptocurrencies" class="width15 bg-gray-800 overflow-y-auto max-h-[9rem] z-50 scrollbar-thin scrollbar-thumb-gray-700 scrollbar-track-gray-800">
            <!-- Wyświetlanie kryptowalut -->
            <ul class="text-sm text-white">
              <li v-for="(crypto, index) in filteredCryptocurrencies" :key="crypto.id" class="flex items-center h-10 px-4">
                <img :src="crypto.icon" alt="Cryptocurrency" class="h-8 mr-2">
                <p>{{ crypto.shortcut }} {{ crypto.name }}</p>
              </li>
            </ul>
          </div>
      
    </header>
    <main>
      <router-view></router-view>
    </main>
  </div>
</template>
<script setup>
import { ref, onMounted } from 'vue';

// Lista kryptowalut
const cryptocurrencies = ref([
  { id: 1, name: 'Bitcoin', shortcut: 'BTC', icon: '/btc.png' },
  { id: 2, name: 'Ethereum', shortcut: 'ETH', icon: '/eth.png' },
  { id: 3, name: 'XRP', shortcut: 'XRP', icon: '/xrp.png' },
  { id: 4, name: 'Internet Computer', shortcut: 'ICP', icon: '/icp.png' },
  // Dodaj pozostałe kryptowaluty tutaj...
]);

// Stan filtrujący listę kryptowalut
const showCryptocurrencies = ref(false);
const searchQuery = ref('');
const filteredCryptocurrencies = ref([...cryptocurrencies.value]);

// Funkcje do obsługi interakcji użytkownika
const toggleCryptocurrencies = () => {
  showCryptocurrencies.value = !showCryptocurrencies.value;
};

const filterCryptocurrencies = () => {
  const query = searchQuery.value.toLowerCase();
  filteredCryptocurrencies.value = cryptocurrencies.value.filter(crypto =>
    crypto.name.toLowerCase().startsWith(query) || crypto.shortcut.toLowerCase().startsWith(query)
  );
};
</script>

<style scoped>
/* scrollbar style*/
::-webkit-scrollbar {
  width: 8px;
}

::-webkit-scrollbar-thumb {
  background-color: rgba(55, 65, 81, 1);
  border-radius: 4px;
}

::-webkit-scrollbar-track {
  background-color: rgba(75, 85, 101, 1);
}

/* Stylowanie scrollbarów */
.scrollbar-thin {
  scrollbar-width: thin;
  scrollbar-color: rgba(55, 65, 81, 1) rgba(75, 85, 101, 1);
}

.scrollbar-thin:hover {
  scrollbar-color: rgba(35, 45, 61, 1) rgba(55, 65, 81, 1);
}

ul {
  list-style-type: none;
  padding: 0;
  margin: 0;
}

li {
  cursor: pointer;
}
.width15{
  width: 15%;
}
</style>