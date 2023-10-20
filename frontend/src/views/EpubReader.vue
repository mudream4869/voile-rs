<template>
  <v-app style='height: 100vh'>
    <vue-reader :url="content_src_url" :epubInitOptions='{ openAs: "epub" }' />
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
      },

      content_idx: 0,
      book_id: '',
    }
  },
  computed: {
    content_src_url() {
      return getContentURL(this.book_id, this.content_idx)
    },
  },
  created() {
    const route = useRoute()
    this.content_idx = parseInt(route.params.content_idx)
    this.book_id = route.params.book_id

    // fetch on init
    this.fetchBook()
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
    async UpdateContentIDX() {
      this.$router.push({
        name: 'pdf_reader',
        params: {
          book_id: this.book_id,
          content_idx: this.content_idx,
          paging: 0,
        }
      })

      setBookProc(this.book_id, this.content_idx, this.paging)
    },
  },
}
</script>
