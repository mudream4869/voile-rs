<template>
  <v-app style='height: 100vh'>
    <vue-reader :location='progress' @update:location='updateProgress' :url="content_src_url"
      :epubInitOptions='{ openAs: "epub" }' />
  </v-app>
</template>

<script>
import { getBook, setBookProgress, getContentURL } from '@/api/books'
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

      progress: '',
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
    if (route.params.progress != '0') {
      this.progress = decodeURIComponent(route.params.progress)
    }

    // fetch on init
    this.fetchBook()

    setBookProgress(this.book_id, this.content_idx, this.progress)
  },
  methods: {
    async fetchBook() {
      this.book = await getBook(this.book_id)
    },
    updateProgress(epubcfi) {
      this.$router.push({
        name: 'epub_reader',
        params: {
          book_id: this.book_id,
          content_idx: this.content_idx,
          progress: encodeURIComponent(epubcfi),
        }
      })
      this.progress = epubcfi
      setBookProgress(this.book_id, this.content_idx, this.progress)
    },
  },
}
</script>
