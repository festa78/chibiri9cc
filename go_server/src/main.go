package main

import (
	"fmt"
	"io/ioutil"
	"net/http"
)

func test(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintf(w, "Hello, World in go test\n")

	resp, err := http.Get("http://nginx-srv/compilerapi/")
	if err != nil {
		fmt.Fprintf(w, fmt.Sprintf("fail on compilerapi: %s\n", err))
		return
	}

	defer resp.Body.Close()
	body, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		fmt.Fprintf(w, fmt.Sprintf("fail on compilerapi: %s\n", err))
		return
	}

	fmt.Fprintf(w, string(body))
}

func handler(w http.ResponseWriter, r *http.Request) {
	resp, err := http.Post("http://nginx-srv/compilerapi/compile", "application/json", r.Body)
	if err != nil {
		fmt.Fprintf(w, fmt.Sprintf("fail on calling compilerapi: %s\n", err))
		return
	}

	defer resp.Body.Close()
	body, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		fmt.Fprintf(w, fmt.Sprintf("fail on reading output of compilerapi: %s\n", err))
		return
	}

	fmt.Fprintf(w, "%s", string(body))
}

func main() {
	// http.HandleFunc("/", test) // ハンドラを登録してウェブページを表示させる
	http.HandleFunc("/", handler) // ハンドラを登録してウェブページを表示させる
	http.ListenAndServe(":8080", nil)
}
