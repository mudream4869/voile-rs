<template>
  <v-app>
    <v-breadcrumbs divider="-" :items="breadcrumbsItems">
    </v-breadcrumbs>
    <v-container class="ma-md-2">
      <v-dialog v-model="edit_dialog">
        <v-card>
          <v-card-title>
            <span class="text-h5"> 編輯 {{ book.title }} </span>
          </v-card-title>
          <v-card-text>
            <v-container fluid>
              <v-row>
                <v-col cols="12">
                  <v-text-field label="標題" required v-model="book.title"></v-text-field>
                  <v-text-field label="作者" v-model="book.author"></v-text-field>
                  <v-autocomplete label="分類" v-model="book.book_type" :items="books_types"></v-autocomplete>
                </v-col>
                <v-col cols="12">
                  <v-chip-group>
                    <v-chip v-for="(tag, i) in book.tags" :key="tag"
                            @click="book.tags.splice(i, 1)"> {{ tag }} </v-chip>
                    <v-autocomplete
                      density="compact"
                      variant="solo"
                      append-inner-icon="mdi-plus"
                      label="想要新增的標籤"
                      single-line
                      hide-details
                      v-model="input_tag"
                      :items="books_tags"
                      @click:append-inner="addTag"
                    ></v-autocomplete>
                  </v-chip-group>
                </v-col>
              </v-row>
            </v-container>
          </v-card-text>
          <v-card-actions>
            <v-spacer></v-spacer>
            <v-btn color="primary" @click="edit_dialog = false; updateBookDetail();">確認修改</v-btn>
            <v-btn color="warning" @click="edit_dialog = false">取消</v-btn>
          </v-card-actions>
        </v-card>
      </v-dialog>
      <v-row>
        <v-col cols="3">
          <v-img
            v-if="book.book_cover"
            class="align-end text-white"
            :src="`/api/books/${book.book_id}/book_cover`"
            height="200"
            cover>
          </v-img>
          <v-img
            v-else
            class="align-end text-white"
            src="https://via.placeholder.com/200"
            height="200"
            cover>
          </v-img>
        </v-col>
        <v-col cols="9">
          <h1>
            {{ book.title }}
            <v-btn variant="tonal" size="small" @click="edit_dialog = true"><v-icon>mdi-pencil</v-icon> 編輯</v-btn>
          </h1> 

          <div v-if="book.tags">
            <v-chip class="ma-2" label
                    v-for="tag in book.tags" :key="tag"> {{ tag }} </v-chip>
          </div>

          <p v-if="book.author"> 作者: {{ book.author }} </p>

          <v-btn variant="outlined" v-if="book_proc.content_idx >= 0"
                 :to="{ name: 'content', params: {book_id, content_idx: book_proc.content_idx, paging: book_proc.paging}}">
            繼續閱讀: {{ book.content_titles[book_proc.content_idx] }}
          </v-btn>

          <v-btn variant="outlined" v-if="book_proc.content_idx == -1"
                 :to="{ name: 'content', params: {book_id, content_idx: 0, paging: 0}}">
            開始閱讀
          </v-btn>
        </v-col>
      </v-row>
      <v-row>
        <v-col cols="4" v-for="(c, idx) in book.content_titles" :key="idx">
          <v-btn variant="text" :to="{ name: 'content', params: {book_id, content_idx: idx, paging: 0}}">
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
          content_titles: [],
          tags: null,
          author: null,
          book_type: null,
        },

        books_tags: [],
        books_types: [],

        book_id: '',
        book_proc: {
          content_idx: -1,
          paging: 0,
        },

        edit_dialog: false,

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
    },
    methods: {
      async fetchData() {
        this.book = (await (await fetch(`/api/books/${this.book_id}`)).json())
        this.books_tags = (await (await fetch(`/api/books_tags`)).json())
        this.books_types = (await (await fetch(`/api/books_types`)).json())
        fetch(`/api/user/book_proc/${this.book_id}`).then(async res => {
          if (res.status == 200) {
            this.book_proc = await res.json()
          }
        })
      },
      async updateBookDetail() {
        await fetch(`/api/books/${this.book_id}`, {
          method: 'POST',
          headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json',
          },
          body: JSON.stringify({
            title: this.book.title,
            author: this.book.author,
            tags: this.book.tags,
            book_type: this.book.book_type,
          }),
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
