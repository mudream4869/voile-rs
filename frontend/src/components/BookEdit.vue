<template>
  <v-app>
    <v-breadcrumbs divider="-" :items="breadcrumbsItems">
    </v-breadcrumbs>
    <v-container class="ma-md-2">
      <v-container fluid>
        <v-row>
          <v-col cols="12">
            <v-text-field label="標題" required v-model="book.title"></v-text-field>
            <v-text-field label="作者" v-model="book.author"></v-text-field>
            <v-autocomplete label="分類" v-model="book.book_type" :items="books_types"></v-autocomplete>
          </v-col>
          <v-col cols="12">
            <v-chip-group>
              <v-chip v-for="(tag, i) in book.tags" :key="tag" @click="book.tags.splice(i, 1)"> {{ tag }} </v-chip>
              <v-autocomplete density="compact" variant="solo" append-inner-icon="mdi-plus" label="想要新增的標籤" single-line
                hide-details v-model="input_tag" :items="books_tags" @click:append-inner="addTag"></v-autocomplete>
            </v-chip-group>
          </v-col>
        </v-row>
      </v-container>
      <v-spacer></v-spacer>
      <v-btn color="primary" @click="updateBookDetail();">確認修改</v-btn>
      <v-btn color="warning" @click="$router.go(-1)">取消</v-btn>
    </v-container>
  </v-app>
</template>

<script>
import { useRoute } from 'vue-router'
import { updateBookDetail, getBook, getAllTags, getAllTypes, getBookCoverURL } from '@/api/books'

export default {
  data: () => {
    return {
      book: {
        title: '',
        content_titles: [],
        tags: null,
        author: null,
        book_type: null,
      },

      books_tags: [],
      books_types: [],

      book_id: '',

      input_tag: '',
    }
  },
  created() {
    const route = useRoute()
    this.book_id = route.params.book_id

    // fetch on init
    this.fetchData()
  },
  computed: {
    bookCoverURL() {
      return getBookCoverURL(this.book, 200)
    },
    breadcrumbsItems() {
      return [
        {
          title: '書櫃',
          link: true,
          disabled: false,
          to: {
            name: 'books',
          },
        },
        {
          title: this.book.title,
          link: true,
          disabled: false,
          to: {
            name: 'book',
            params: {
              book_id: this.book_id,
            },
          },
        },
        {
          title: '編輯',
          disabled: false,
        },
      ]
    },
  },
  methods: {
    async fetchData() {
      this.book = await getBook(this.book_id)
      this.books_tags = await getAllTags()
      this.books_types = await getAllTypes()
    },
    async updateBookDetail() {
      await updateBookDetail(this.book_id, {
        title: this.book.title,
        author: this.book.author,
        tags: this.book.tags,
        book_type: this.book.book_type,
      })
      this.$router.push({
        name: 'book', params: {
          book_id: this.book_id
        }
      })
    },
    addTag() {
      if (this.book.tags) {
        this.book.tags.push(this.input_tag);
      } else {
        this.book.tags = [this.input_tag];
      }
      this.input_tag = '';
    }
  },
}
</script>
