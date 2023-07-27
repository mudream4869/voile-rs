<template>
  <v-app tabindex="0" @keyup.arrow-left="previous_content" @keyup.arrow-right="next_content">
    <v-dialog v-model="configDialog">
      <v-card>
        <v-card-text>
          <v-slider min="60" max="100" step="4" show-ticks v-model="config.imageSize">
            <template v-slot:prepend>
              Image Width:
            </template>
            <template v-slot:append>
              {{ config.imageSize }} %
            </template>
          </v-slider>
          <v-slider min="20" max="100" step="10" show-ticks v-model="config.fontSize">
            <template v-slot:prepend>
              Font Size:
            </template>
            <template v-slot:append>
              {{ config.fontSize }} px
            </template>
          </v-slider>
        </v-card-text>
        <v-card-actions>
          <v-btn color="primary" block @click="configDialog = false">Close Dialog</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <div v-if="is_text" class="ma-md-2" :style="'font-size:' + config.fontSize + 'px;'" @dblclick="configDialog = true">
      <h1> {{ content_title }} </h1>
      <div v-if="paging_max > 1">
        <v-pagination @update:modelValue="updatePaging" v-model="paging" :length="paging_max" :start="0"
          :total-visible="10">
        </v-pagination>
      </div>

      <span v-if="current_page_lines > 0" v-for="idx in current_page_lines" :key="idx">
        {{ content_lines[idx - 1 + paging * paging_line] }}<br />
      </span>

      <div v-if="paging_max > 1">
        <v-pagination @update:modelValue="updatePaging" v-model="paging" :length="paging_max" :start="0"
          :total-visible="10">
        </v-pagination>
      </div>
    </div>
    <div v-if="is_image" class="mx-auto">
      <v-img :width="config.imageSize + 'vw'" :src="content_src_url" @dblclick="configDialog = true"></v-img>
      <!-- prefetch next image -->
      <v-img :src="next_content_src_url" hidden></v-img>
    </div>
  </v-app>
</template>

<script>
import { getBook, setBookProc, getContentURL } from '@/api/books'

import { useRoute } from 'vue-router'

function is_text_suffix(filename) {
  return filename.endsWith('.txt')
}

function is_image_suffix(filename) {
  return filename.endsWith('.png') || filename.endsWith('.jpg') || filename.endsWith('.gif') || filename.endsWith('.jpeg')
}

export default {
  data: () => {
    return {
      book: {
        title: '',
        content_titles: [],
        tags: null,
        author: null,
      },

      content_idx: 0,
      book_id: '',
      content: '',
      content_lines: [],

      // split txt if it is large
      paging_line: 100,
      // paging in [0, paging_max)
      paging: 0,
      paging_max: 1,

      configDialog: false,
      config: {
        imageSize: 90,
        fontSize: 40,
      },
    }
  },
  computed: {
    content_title() {
      return this.book.content_titles[this.content_idx]
    },
    is_text() {
      if (this.book.content_titles.length > this.content_idx) {
        return this.book.content_titles && is_text_suffix(this.book.content_titles[this.content_idx])
      }
      return false
    },
    is_image() {
      if (this.book.content_titles.length > this.content_idx) {
        return this.book.content_titles && is_image_suffix(this.book.content_titles[this.content_idx])
      }
      return false
    },
    content_src_url() {
      return getContentURL(this.book_id, this.content_idx)
    },
    next_content_src_url() {
      return getContentURL(this.book_id, this.content_idx + 1)
    },
    current_page_lines() {
      return Math.min((this.paging + 1) * this.paging_line, this.content_lines.length) - this.paging * this.paging_line
    }
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
      if (this.is_text) {
        this.content = (await (await fetch(this.content_src_url)).text())
        this.content_lines = this.content.split('\n')
        this.paging_max = Math.ceil(this.content_lines.length / this.paging_line)
      }
    },
    updatePaging(nextPaging) {
      if (nextPaging) {
        this.paging = nextPaging
      }
      this.$router.push({
        name: 'mixture_reader',
        params: {
          book_id: this.book_id,
          content_idx: this.content_idx,
          paging: this.paging,
        }
      })
      setBookProc(this.book_id, this.content_idx, this.paging)
    },
    async UpdateContentIDX() {
      if (this.is_text) {
        this.content = (await (await fetch(this.content_src_url)).text())
        this.content_lines = this.content.split('\n')
        this.paging_max = Math.ceil(this.content_lines.length / this.paging_line)
      }

      this.$router.push({
        name: 'mixture_reader',
        params: {
          book_id: this.book_id,
          content_idx: this.content_idx,
          paging: this.paging,
        }
      })

      setBookProc(this.book_id, this.content_idx, this.paging)
    },
    previous_content() {
      if (this.paging > 0) {
        this.paging -= 1
        this.updatePaging()
        return
      }
      if (this.content_idx > 0) {
        this.content_idx -= 1
        this.paging = 0
        this.UpdateContentIDX()
      }
    },
    next_content() {
      if (this.paging + 1 < this.paging_max) {
        this.paging += 1
        this.updatePaging()
        return
      }

      if (this.content_idx + 1 < this.book.content_titles.length) {
        this.content_idx += 1
        this.paging = 0
        this.UpdateContentIDX()
      }
    },
  },
}
</script>
