<template>
  <v-app>
    <v-breadcrumbs divider="-" :items="breadcrumbsItems">
    </v-breadcrumbs>
    <v-dialog v-model="changeBookCoverDialog" width="50%">
      <v-card>
        <v-tabs v-model="tab" bg-color="primary">
          <v-tab value="localfile">
            上傳本機檔案
          </v-tab>
        </v-tabs>
        <v-card-text>
          <v-window v-model="tab">
            <v-window-item value="localfile">
              <v-file-input accept="image/png, image/jpeg" label="上傳封面" show-size
                @change="uploadBookCover($event)"></v-file-input>
            </v-window-item>
          </v-window>
        </v-card-text>
      </v-card>
    </v-dialog>
    <v-container class="ma-md-2">
      <v-container fluid>
        <v-row>
          <v-col cols="3">
            <v-hover>
              <template v-slot:default="{ isHovering, props }">
                <v-card :elevation="isHovering || changeBookCoverDialog ? 12 : 2" v-bind="props">
                  <v-img :src="bookCoverURL" height="400" :class="{ 'on-hover': isHovering || changeBookCoverDialog }"
                    cover>
                  </v-img>
                  <div align="center">
                    <v-btn variant="text" color="red" icon="mdi-delete"></v-btn>
                    <v-btn variant="text" color="green" icon="mdi-pencil" @click="changeBookCoverDialog = true"></v-btn>
                  </div>
                </v-card>
              </template>
            </v-hover>
          </v-col>
          <v-col cols="9">
            <v-text-field label="標題" required v-model="book.title"></v-text-field>
            <v-text-field label="作者" v-model="book.author"></v-text-field>
            <v-autocomplete label="分類" v-model="book.book_type" :items="books_types"></v-autocomplete>
            <v-chip-group>
              <v-chip v-for="(tag, i) in book.tags" :key="tag" @click="book.tags.splice(i, 1)"> {{ tag }} </v-chip>
              <v-autocomplete density="compact" variant="solo" append-inner-icon="mdi-plus" label="想要新增的標籤" single-line
                hide-details v-model="input_tag" :items="books_tags" @click:append-inner="addTag"></v-autocomplete>
            </v-chip-group>
            <v-divider class="ma-md-4"></v-divider>
            <v-btn class="ma-md-2" color="primary" @click="updateBookDetail();">確認修改</v-btn>
            <v-btn class="ma-md-2" color="warning" @click="$router.go(-1)">取消</v-btn>
          </v-col>
        </v-row>
      </v-container>
    </v-container>
  </v-app>
</template>

<script>
import { useRoute } from 'vue-router'
import { updateBookDetail, getBook, getAllTags, getAllTypes, getBookCoverURL, uploadBookCover } from '@/api/books'

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

      changeBookCoverDialog: false,

      tab: '',
      cacheKey: 1,
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
      return getBookCoverURL(this.book, 200) + '?c=' + this.cacheKey.toString()
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
    },
    async uploadBookCover(event) {
      await uploadBookCover(this.book_id, event.target.files[0])
      this.changeBookCoverDialog = false
      this.cacheKey += 1
    },
  },
}
</script>
<style scoped>
.v-img {
  opacity: 0.6;
  transition: opacity .4s ease-in-out;
}

.v-img:not(.on-hover) {
  opacity: 1.0;
}
</style>
