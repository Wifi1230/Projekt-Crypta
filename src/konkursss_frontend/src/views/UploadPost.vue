<template>
  <div class="upload-post-container flex justify-center items-center min-h-screen bg-gray-900">
    <div class="upload-post-box bg-gray-800 border border-gray-700 rounded-lg p-6 w-2/3">
      <div class="flex justify-between mb-4">
        <div class="w-1/3">
          <select v-model="selectedCrypto" class="w-full p-2 bg-gray-700 border border-gray-600 rounded text-white">
            <option v-for="crypto in cryptocurrencies" :key="crypto.id" :value="crypto">
              {{ crypto.shortcut }} - {{ crypto.name }}
            </option>
          </select>
        </div>
        <div class="w-1/3">
          <select v-model="prediction" class="w-full p-2 bg-gray-700 border border-gray-600 rounded text-white">
            <option value="Bullish">Bullish</option>
            <option value="Info">Info</option>
            <option value="Bearish">Bearish</option>
          </select>
        </div>
      </div>
      <textarea 
        v-model="postText"
        class="w-full p-4 bg-gray-700 border border-gray-600 rounded-lg text-white h-40"
        placeholder="Napisz coś..."
      ></textarea>
      <button 
        @click="submitPost" 
        class="mt-4 px-6 py-2 bg-gray-700 text-white rounded-lg hover:bg-blue-700"
      >
        Submit Post
      </button>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import { konkursss_backend } from 'declarations/konkursss_backend/index';
import { userStore } from '../store'; // upewnij się, że ścieżka jest poprawna

const router = useRouter();
const postText = ref('');
const selectedCrypto = ref(null);
const prediction = ref('');
const cryptocurrencies = ref([]);

const downloadCryptocurrencies = async () => {
  cryptocurrencies.value = await konkursss_backend.get_all_cryptos();
};

const submitPost = async () => {
  try {
    if (!userStore.username) {
      console.error('User is not logged in');
      return;
    }

    const postData = {
      post_text: postText.value,
      selected_crypto: selectedCrypto.value ? selectedCrypto.value.shortcut : '',
      prediction: prediction.value,
      likes: 0,
      dislikes: 0,
      username: userStore.username // Dodajemy nazwę użytkownika
    };

    const response = await konkursss_backend.dodaj_wpis(postData);
    console.log('Response from backend:', response);
    router.push('/');
  } catch (error) {
    console.error('Error sending data to backend:', error);
  }
};

downloadCryptocurrencies();
</script>

<style scoped>
.upload-post-container {
  padding: 20px;
}

.upload-post-box {
  box-shadow: 0 0 10px rgba(0, 0, 0, 0.5);
}

textarea::placeholder {
  color: rgba(255, 255, 255, 0.5);
}

select {
  appearance: none;
  -webkit-appearance: none;
  -moz-appearance: none;
}
</style>
