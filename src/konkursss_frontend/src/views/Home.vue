<template>
  <div class="bg-slate-900 min-h-screen h-full overflow-hidden">
    <main>
      <div class="custom-height"></div>
      <div class="flex justify-center w-full">
        <div class="w-2/3">
          <div class="text-white grid gap-4 my-10">
            <!-- Wyświetlanie wpisów -->
            <div v-for="(wpis, index) in filteredWpisy" :key="index" class="post drop-shadow-xl bg-gray-700 p-4 relative rounded-lg">
              <div class="flex items-start mb-2">
                <div class="text-sm text-gray-400 mr-4">
                  <span class="block">{{ wpis.username }}</span>
                  <span class="block">{{ wpis.selected_crypto }}</span>
                  <span 
                    class="block" 
                    :class="{
                      'text-green-500': wpis.prediction === 'Bullish',
                      'text-red-500': wpis.prediction === 'Bearish',
                      'text-gray-400': wpis.prediction === 'Info'
                    }"
                  >
                    {{ wpis.prediction }}
                  </span>
                </div>
                <p class="text-white">{{ wpis.post_text }}</p>
              </div>
              <div class="flex justify-between items-center mt-4">
                <button class="bg-red-600 rounded text-white px-4 py-2" @click="deleteWpis(index)">Usuń</button>
                <div class="flex items-center space-x-4">
                  <div class="flex items-center">
                    <img src="/like.png" alt="Like" class="w-6 h-6 cursor-pointer transition-transform transform hover:scale-110" @click="likePost(index)">
                    <span class="ml-2 text-white">{{ wpis.likes }}</span>
                  </div>
                  <div class="flex items-center">
                    <img src="/dislike.png" alt="Dislike" class="w-6 h-6 cursor-pointer transition-transform transform hover:scale-110" @click="dislikePost(index)">
                    <span class="ml-2 text-white">{{ wpis.dislikes }}</span>
                  </div>
                  <div class="flex items-center">
                    <span class="cursor-pointer" @click="toggleComments(index)">⬇️ ({{ wpis.comments.length }})</span>
                  </div>
                </div>
              </div>
              <!-- Komentarze -->
              <div v-if="wpis.showComments" class="mt-4">
                <div class="flex items-start mb-2">
                  <textarea v-model="newCommentText[index]" class="w-full p-2 bg-gray-700 border border-gray-600 rounded text-white" placeholder="Dodaj komentarz..."></textarea>
                  <button @click="addComment(index)" class="bg-blue-600 text-white rounded px-4 py-2 ml-2">Dodaj</button>
                </div>
                <div v-for="comment in wpis.comments" :key="comment.id" class="mb-2 p-2 bg-gray-800 rounded">
                  <p class="text-white">{{ comment.text }}</p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup>
import { ref, onMounted, watch } from 'vue';
import { konkursss_backend } from 'declarations/konkursss_backend/index';

const props = defineProps(['selectedCrypto']);
const userId = ref(''); // Załaduj ID użytkownika w odpowiedni sposób

const wpisy = ref([]);
const filteredWpisy = ref([]);
const newCommentText = ref([]);

const pobierzWpisy = async () => {
  wpisy.value = await konkursss_backend.odczytaj_wpisy();
  wpisy.value.forEach(wpis => {
    wpis.showComments = false;
    wpis.comments = [];
  });
  filterPostsByCrypto();
};

const filterPostsByCrypto = () => {
  if (props.selectedCrypto && wpisy.value.length > 0) {
    filteredWpisy.value = wpisy.value.filter(wpis => wpis.selected_crypto.toLowerCase() === props.selectedCrypto.toLowerCase());
  } else {
    filteredWpisy.value = [...wpisy.value];
  }
};

const userHasLiked = async (postId) => {
  return await konkursss_backend.user_has_liked(userId.value, postId);
};

const userHasDisliked = async (postId) => {
  return await konkursss_backend.user_has_disliked(userId.value, postId);
};

const likePost = async (index) => {
  try {
    const postId = BigInt(index);
    const hasLiked = await userHasLiked(postId);
    const hasDisliked = await userHasDisliked(postId);

    if (hasLiked) {
      // Jeśli użytkownik już polubił post, anuluj "like"
      await konkursss_backend.like_wpis(userId.value, postId);
    } else {
      // Jeśli użytkownik już odrzucił post, zamień "dislike" na "like"
      if (hasDisliked) {
        await konkursss_backend.dislike_wpis(userId.value, postId);
      }
      await konkursss_backend.like_wpis(userId.value, postId);
    }

    await pobierzWpisy();
  } catch (e) {
    console.error('Failed to like post:', e);
  }
};

const dislikePost = async (index) => {
  try {
    const postId = BigInt(index);
    const hasLiked = await userHasLiked(postId);
    const hasDisliked = await userHasDisliked(postId);

    if (hasDisliked) {
      // Jeśli użytkownik już odrzucił post, anuluj "dislike"
      await konkursss_backend.dislike_wpis(userId.value, postId);
    } else {
      // Jeśli użytkownik już polubił post, zamień "like" na "dislike"
      if (hasLiked) {
        await konkursss_backend.like_wpis(userId.value, postId);
      }
      await konkursss_backend.dislike_wpis(userId.value, postId);
    }

    await pobierzWpisy();
  } catch (e) {
    console.error('Failed to dislike post:', e);
  }
};

const startAutoRefresh = () => {
  setInterval(async () => {
    await pobierzWpisy();
  }, 5000); // Odświeżaj co 5 sekund
};

onMounted(async () => {
  await pobierzWpisy();
  startAutoRefresh();
});
</script>

<style>
.custom-height {
  height: 20vh; /* Adjust the value based on your needs */
}
</style>
