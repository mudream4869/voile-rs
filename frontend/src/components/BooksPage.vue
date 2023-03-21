<template>
  <v-app>
    <v-container fluid>
      <v-row>
        <v-col>
          <v-chip class="ma-2"
                  v-for="tag in tags"
                  :key="tag"
                  :color="used_tags.has(tag) ? 'green': 'default'"
                  @click="toggleTag(tag)"
                  label> {{ tag }} </v-chip>
        </v-col>
      </v-row>
      <v-row dense>
        <v-col v-for="book in show_books" :key="book.book_id" :cols="4">
          <v-card outlined shaped class="mx-auto ma-md-2" width="400">
            <v-img
              v-if="book.book_cover"
              class="align-end text-white"
              height="200"
              :src="`/api/books/${book.book_id}/book_cover`"
              cover>
            </v-img>
            <v-card-title>
              {{ book.title }}
            </v-card-title>

            <div v-if="book.tags">
              <v-chip class="ma-2" label
                      v-for="tag in book.tags" :key="tag"> {{ tag }} </v-chip>
            </div>

            <v-card-actions>
              <v-btn outlined :to="{ name: 'book', params: { book_id: book.book_id }}">
                閱讀
              </v-btn>
            </v-card-actions>
          </v-card>
        </v-col>
      </v-row>
    </v-container>

    
  </v-app>
</template>

<script>
  export default {
    data: () => {
      return {
        books: [],
        tags: [],
        used_tags: new Set(),
      }
    },
    created() {
      // fetch on init
      this.fetchData()
    },
    computed: {
      show_books() {
        if (this.used_tags.size == 0) {
          return this.books
        }

        return this.books.filter(book => {
          if (!book.tags) {
            return false
          }
          var all_tag = true
          this.used_tags.forEach(tag => {
            if (!book.tags_set.has(tag)) {
              all_tag = false
            }
          })
          return all_tag
        })
      }
    },
    methods: {
      async fetchData() {
        this.books = (await (await fetch('/api/books')).json()).books.map(book => {
          book.tags_set = new Set(book.tags || [])
          return book
        })
        this.tags = [...new Set(this.books.map(book => book.tags || []).flat())]
      },
      toggleTag(tag) {
        if (this.used_tags.has(tag)) {
          this.used_tags.delete(tag)
        } else {
          this.used_tags.add(tag)
        }
      }
    },
  }
</script>
