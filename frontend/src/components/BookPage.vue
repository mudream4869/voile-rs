<template>
  <v-app>
    <v-breadcrumbs divider="-" :items="breadcrumbsItems">
    </v-breadcrumbs>
    <v-container class="ma-md-2">
      <v-row>
        <v-col cols="3">
          <v-img class="align-end text-white" :src="bookCoverURL" height="400" cover>
          </v-img>
        </v-col>
        <v-col cols="9">
          <h1>
            {{ book.title }}
          </h1>

          <div v-if="book.tags">
            <v-chip class="ma-2" label v-for="tag in book.tags" :key="tag"> {{ tag }} </v-chip>
          </div>

          <p v-if="book.author"> 作者: {{ book.author }} </p>

          <v-btn variant="outlined" v-if="book_proc.content_idx >= 0" target="_blank"
            :to="{ name: reader_name, params: { book_id, content_idx: book_proc.content_idx, paging: book_proc.paging } }">
            繼續閱讀: {{ book.content_titles[book_proc.content_idx] }}
          </v-btn>

          <v-btn variant="outlined" v-if="book_proc.content_idx == -1" target="_blank"
            :to="{ name: reader_name, params: { book_id, content_idx: 0, paging: 0 } }">
            開始閱讀
          </v-btn>

          <p class="my-2">
            <v-btn variant="tonal" size="small" :to="{ name: 'edit_book', params: { book_id } }">
              <v-icon>mdi-pencil</v-icon>編輯
            </v-btn>
          </p>
        </v-col>
      </v-row>
      <v-row v-if="is_mixture_reader">
        <v-col cols="4" v-for="(c, idx) in book.content_titles" :key="idx">
          <v-btn variant="text" target="_blank"
            :to="{ name: 'mixture_reader', params: { book_id, content_idx: idx, paging: 0 } }">
            {{ c }}
          </v-btn>
        </v-col>
      </v-row>
    </v-container>
  </v-app>
</template>

<script>
import { useRoute } from 'vue-router'
import { getBook, getBookProc, getBookCoverURL } from '@/api/books'

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

      book_id: '',
      book_proc: {
        content_idx: -1,
        paging: 0,
      },
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
      return getBookCoverURL(this.book, 400)
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
          disabled: false,
        },
      ]
    },
    reader_name() {
      if (this.is_mixture_reader) {
        return 'mixture_reader'
      }
      if (this.is_pdf_reader) {
        return 'pdf_reader'
      }
      return 'mixture_reader'
    },
    is_mixture_reader() {
      return !this.is_pdf_reader;
    },
    is_pdf_reader() {
      return this.book.content_titles.length == 1 && this.book.content_titles[0].endsWith('.pdf')
    }
  },
  methods: {
    async fetchData() {
      this.book = await getBook(this.book_id)

      let book_proc = await getBookProc(this.book_id)
      if (book_proc) {
        this.book_proc = book_proc
      }

      document.title += ' | ' + this.book.title
    },
  },
}
</script>
