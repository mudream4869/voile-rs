<template>
  <v-app style='height: 100vh'>
    <vue-reader :url="content_src_url" />
  </v-app>
</template>

<script>
import { getBook, setBookProc, getContentURL } from '@/api/books'
import VueReader from 'vue-reader'

import { useRoute } from 'vue-router'

export default {
  components: {
    VueReader
  },
  data: () => {
    return {
      book: {
        title: '',
        content_titles: [],
      },

      content_idx: 0,
      book_id: '',

      // paging in [0, paging_max)
      paging: 0,
      paging_max: 1,
    }
  },
  computed: {
    content_title() {
      return this.book.content_titles[this.content_idx]
    },
    content_src_url() {
      return getContentURL(this.book_id, this.content_idx)
    },
  },
  created() {
    const route = useRoute()
    this.content_idx = parseInt(route.params.content_idx)
    this.paging = parseInt(route.params.paging)
    this.book_id = route.params.book_id

    // fetch on init
    this.fetchBook()

    setBookProc(this.book_id, this.content_idx, this.paging)
  },
  updated() {
    const new_content_idx = parseInt(this.$route.params.content_idx);
    if (new_content_idx != this.content_idx) {
      this.content_idx = new_content_idx
      this.UpdateContentIDX()
    }
  },
  methods: {
    async fetchBook() {
      this.book = await getBook(this.book_id)
    },
    updatePaging(nextPaging) {
      if (nextPaging) {
        this.paging = nextPaging
      }
      this.$router.push({
        name: 'epub_reader',
        params: {
          book_id: this.book_id,
          content_idx: this.content_idx,
          paging: this.paging,
        }
      })
      setBookProc(this.book_id, this.content_idx, this.paging)
    },
    async UpdateContentIDX() {
      this.$router.push({
        name: 'pdf_reader',
        params: {
          book_id: this.book_id,
          content_idx: this.content_idx,
          paging: this.paging,
        }
      })

      setBookProc(this.book_id, this.content_idx, this.paging)
    },
  },
}
</script>
