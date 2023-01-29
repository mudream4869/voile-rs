<template>
  <v-app>
    <v-container class="ma-md-2">
      <v-row>
        <v-col cols="12">
          <h1> {{ book.title }} </h1>

          <div v-if="book.tags">
            <v-chip class="ma-2" label
                    v-for="tag in book.tags" :key="tag"> {{ tag }} </v-chip>
          </div>

          <p v-if="book.author"> 作者: {{ book.author }} </p>

          <v-btn variant="outlined" v-if="last_content_idx >= 0"
                 :to="{ name: 'content', params: {book_id, content_idx: last_content_idx}}">
            繼續閱讀: {{ book.contents[last_content_idx] }}
          </v-btn>

          <v-btn variant="outlined" v-if="last_content_idx == -1"
                 :to="{ name: 'content', params: {book_id, content_idx: 0}}">
            開始閱讀
          </v-btn>
        </v-col>
      </v-row>
      <v-row>
        <v-col cols="4" v-for="(c, idx) in book.contents" :key="idx">
          <v-btn variant="text" :to="{ name: 'content', params: {book_id, content_idx: idx}}">
            {{ c }}
          </v-btn>
        </v-col>
      </v-row>
    </v-container>
  </v-app>
</template>

<script>
  import { useRoute } from 'vue-router'

  export default {
    data: () => {
      return {
        book: {
          title: '',
          contents: [],
          tags: null,
          author: null,
        },

        book_id: '',
        last_content_idx: -1,
      }
    },
    created() {
      const route = useRoute()
      this.book_id = route.params.book_id

      // fetch on init
      this.fetchData()
    },
    methods: {
      async fetchData() {
        this.book = (await (await fetch(`/api/books/${this.book_id}`)).json())
        fetch(`/api/user/book_proc/${this.book_id}`).then(async res => {
          if (res.status == 200) {
            this.last_content_idx = parseInt(await res.text())
          }
        })
      },
    },
  }
</script>
