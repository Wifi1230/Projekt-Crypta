<template>
  <div class="bg-slate-900 min-h-screen h-full overflow-hidden">
    <main>
      <div class="custom-height"></div>
      <div class="flex justify-center w-full">
        <div class="w-2/3">
          <div class="text-white grid gap-4 my-10">
            <div v-for="(wpis, index) in filteredWpisy" :key="index" class="post drop-shadow-xl bg-gray-700 p-4 relative rounded-lg">
              <div class="flex flex-col mb-2">
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
                <div class="flex-1">
                  <p class="text-white">{{ wpis.post_text }}</p>
                </div>
                <div class="flex items-center ml-4 space-x-4">
                  <div class="flex items-center">
                    <p class="w-6 h-6 cursor-pointer transition-transform transform hover:scale-110" @click="likePost(index)">👍</p>
                    <span class="ml-2 text-white">{{ wpis.likes }}</span>
                  </div>
                  <div class="flex items-center">
                    <p class="w-6 h-6 cursor-pointer transition-transform transform hover:scale-110" @click="dislikePost(index)">👎</p>
                    <span class="ml-2 text-white">{{ wpis.dislikes }}</span>
                  </div>
                  <div class="flex items-center">
                    <span class="cursor-pointer" @click="toggleComments(index)">⬇️ ({{ wpis.comments.length }})</span>
                  </div>
                </div>
              </div>
            </div>
              <!-- Comments Section -->
              <div v-if="wpis.showComments" class="mt-4">
                <div class="flex items-start mb-2">
                  <textarea v-model="newCommentText[index]" class="w-full p-2 bg-gray-700 border border-gray-600 rounded text-white" placeholder="Add comment..."></textarea>
                  <button @click="addComment(index)" class="bg-slate-900 text-white rounded px-4 py-2 ml-2">Add</button>
                </div>
                <div v-for="(comment, commentIndex) in wpis.comments" :key="comment.id" class="mb-2 p-2 bg-gray-700 border border-slate-900 rounded">
                  <p class="text-white"><strong>{{ comment.username }}:</strong> {{ comment.text }}</p>
                  <div class="flex items-center">
                    <p class="w-6 h-6 cursor-pointer transition-transform transform hover:scale-110" @click="likeComment(index, commentIndex)">👍</p>
                    <span class="ml-1 text-white">{{ comment.likes }}</span>
                    <p class="w-6 h-6 cursor-pointer transition-transform transform hover:scale-110" @click="dislikeComment(index, commentIndex)">👎</p>
                    <span class="ml-1 text-white">{{ comment.dislikes }}</span>
                  </div>
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
import { ref, onMounted } from 'vue';
import { konkursss_backend } from 'declarations/konkursss_backend/index';
import { userStore } from '../store'; 


const props = defineProps(['selectedCrypto']);

const wpisy = ref([]);
const filteredWpisy = ref([]);
const newCommentText = ref([]); 


const pobierzWpisy = async () => {
  const fetchedWpisy = await konkursss_backend.odczytaj_wpisy();
  wpisy.value = fetchedWpisy.map((wpis, index) => ({
    ...wpis,
    showComments: false,
    comments: [],
    index 
  }));
  wpisy.value.sort((a, b) => b.index - a.index);

  await Promise.all(wpisy.value.map(async (wpis, index) => {
    const comments = await konkursss_backend.odczytaj_komentarze(wpis.index);
    wpis.comments = comments;
  }));

  filterPostsByCrypto();
};


const filterPostsByCrypto =() => {
  if (props.selectedCrypto && wpisy.value.length > 0) {
    filteredWpisy.value = wpisy.value.filter(wpis => wpis.selected_crypto.toLowerCase() === props.selectedCrypto.toLowerCase());
  } else {
    filteredWpisy.value = [...wpisy.value];
  }
};


const userHasLiked = async (postId) => {
  return await konkursss_backend.user_has_liked(userStore.username, postId);
};


const userHasDisliked = async (postId) => {
  return await konkursss_backend.user_has_disliked(userStore.username, postId);
};


const likePost = async (index) => {
  try {
    const totalPosts = wpisy.value.length;
    const reversedIndex = totalPosts - 1 - index;
    const postId = BigInt(reversedIndex);
    const hasLiked = await userHasLiked(postId);
    const hasDisliked = await userHasDisliked(postId);

    if (hasLiked) {

      await konkursss_backend.like_wpis(userStore.username, postId);
    } else {

      if (hasDisliked) {
        await konkursss_backend.dislike_wpis(userStore.username, postId);
      }
      await konkursss_backend.like_wpis(userStore.username, postId);
    }

    await pobierzWpisy();
  } catch (e) {
    console.error('Failed to like post:', e);
  }
};


const dislikePost = async (index) => {
  try {
    const totalPosts = wpisy.value.length;
    const reversedIndex = totalPosts - 1 - index;
    const postId = BigInt(reversedIndex);
    const hasLiked = await userHasLiked(postId);
    const hasDisliked = await userHasDisliked(postId);

    if (hasDisliked) {

      await konkursss_backend.dislike_wpis(userStore.username, postId);
    } else {

      if (hasLiked) {
        await konkursss_backend.like_wpis(userStore.username, postId);
      }
      await konkursss_backend.dislike_wpis(userStore.username, postId);
    }

    await pobierzWpisy();
  } catch (e) {
    console.error('Failed to dislike post:', e);
  }
};


const toggleComments = (index) => {
  filteredWpisy.value[index].showComments = !filteredWpisy.value[index].showComments;
};


const addComment = async (postIndex) => {
  try {
    const totalPosts = wpisy.value.length;
    const reversedIndex = totalPosts - 1 - postIndex;
    const postId = BigInt(reversedIndex);
    const commentText = newCommentText.value[postIndex];
    
    if (commentText.trim() === '') {
      return;
    }
    if (!userStore.username) {
      console.error('User is not logged in');
      return;
    }
    
    const comment = {
      text: commentText,
      username: userStore.username,
      likes: 0,
      dislikes: 0,
      wpis_index: postId,
    };

    await konkursss_backend.dodaj_komentarz(postId, comment);
    newCommentText.value[postIndex] = '';
    await pobierzWpisy();
  } catch (e) {
    console.error('Failed to add comment:', e);
  }
};


const userHasLikedComment = async (postId, commentId) => {
  return await konkursss_backend.user_has_liked_comment(userStore.username, postId, commentId);
};


const userHasDislikedComment = async (postId, commentId) => {
  return await konkursss_backend.user_has_disliked_comment(userStore.username, postId, commentId);
};


const likeComment = async (postIndex, commentIndex) => {
  try {
    const totalPosts = wpisy.value.length;
    const reversedIndex = totalPosts - 1 - postIndex;
    const postId = BigInt(reversedIndex);
    const commentId = BigInt(commentIndex);
    const hasLiked = await userHasLikedComment(postId, commentId);
    const hasDisliked = await userHasDislikedComment(postId, commentId);

    if (hasLiked) {

      await konkursss_backend.like_comment(userStore.username, postId, commentId);
    } else {

      if (hasDisliked) {
        await konkursss_backend.dislike_comment(userStore.username, postId, commentId);
      }
      await konkursss_backend.like_comment(userStore.username, postId, commentId);
    }

    await pobierzWpisy();
  } catch (e) {
    console.error('Failed to like comment:', e);
  }
};


const dislikeComment = async (postIndex, commentIndex) => {
  try {
    const totalPosts = wpisy.value.length;
    const reversedIndex = totalPosts - 1 - postIndex;
    const postId = BigInt(reversedIndex);
    const commentId = BigInt(commentIndex);
    const hasLiked = await userHasLikedComment(postId, commentId);
    const hasDisliked = await userHasDislikedComment(postId, commentId);

    if (hasDisliked) {

      await konkursss_backend.dislike_comment(userStore.username, postId, commentId);
    } else {

      if (hasLiked) {
        await konkursss_backend.like_comment(userStore.username, postId, commentId);
      }
      await konkursss_backend.dislike_comment(userStore.username, postId, commentId);
    }

    await pobierzWpisy();
  } catch (e) {
    console.error('Failed to dislike comment:', e);
  }
};


const startAutoRefresh = () => {
  setInterval(async () => {
    await pobierzWpisy();
  }, 20000); 
};


onMounted(async () => {
  await pobierzWpisy();
  startAutoRefresh();
});
</script>

<style>
.custom-height {
  height: 20vh;
}
</style>
