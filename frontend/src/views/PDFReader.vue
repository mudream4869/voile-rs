<template>
  <v-app tabindex="0" @keyup.arrow-left="previous_content" @keyup.arrow-right="next_content">
    <div v-if="paging_max > 1" class="mx-auto">
      <v-pagination @update:modelValue="updatePaging" v-model="paging" :length="paging_max" :start="0"
        :total-visible="10">
      </v-pagination>
    </div>
    <vue-pdf-embed ref="pdfRef" :source="content_src_url" @rendered="handle_pdf_render" :page="paging + 1" />
  </v-app>
</template>

<script>
import { getBook, setBookProgress, getContentURL } from '@/api/books'
import VuePdfEmbed from 'vue-pdf-embed'

import { useRoute } from 'vue-router'

export default {
  components: {
    VuePdfEmbed
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
    this.book_id = route.params.book_id
    this.content_idx = parseInt(route.params.content_idx)
    if (route.params.progress != '0') {
      this.paging = parseInt(route.params.progress)
    }

    // fetch on init
    this.fetchBook()

    setBookProgress(this.book_id, this.content_idx, this.paging.toString())
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
        name: 'pdf_reader',
        params: {
          book_id: this.book_id,
          content_idx: this.content_idx,
          progress: this.paging,
        }
      })
      setBookProgress(this.book_id, this.content_idx, this.paging.toString())
    },
    previous_content() {
      if (this.paging > 0) {
        this.paging -= 1
        this.updatePaging()
        return
      }
    },
    next_content() {
      if (this.paging + 1 < this.paging_max) {
        this.paging += 1
        this.updatePaging()
        return
      }
    },
    handle_pdf_render() {
      this.paging_max = this.$refs.pdfRef.pageCount
    },
  },
}
</script>
