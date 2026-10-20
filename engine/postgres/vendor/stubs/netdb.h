/* stub for wasi: libpg_query headers reference these but the parser never calls them */
#ifndef STUB_NETDB_H
#define STUB_NETDB_H
#define NI_MAXHOST 1025
#define NI_MAXSERV 32
#define EAI_FAIL -4
struct addrinfo { int ai_flags; int ai_family; int ai_socktype; int ai_protocol; unsigned ai_addrlen; struct sockaddr *ai_addr; char *ai_canonname; struct addrinfo *ai_next; };
#endif
