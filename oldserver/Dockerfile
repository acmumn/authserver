FROM golang:alpine
RUN apk add --no-cache curl git && \
	curl https://raw.githubusercontent.com/golang/dep/master/install.sh | sh && \
	go get github.com/jessevdk/go-assets-builder
WORKDIR /go/src/identity
COPY . .
RUN dep ensure
RUN go-assets-builder assets -o assets.go
RUN go build

FROM alpine:latest
WORKDIR /root
COPY --from=0 /go/src/identity/identity identity-server
CMD ["./identity-server"]
