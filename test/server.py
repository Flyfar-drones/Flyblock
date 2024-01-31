import socket

def handle_client(client_socket):
    # Receive and print data from the client
    data = client_socket.recv(255).decode()
    print(f"Received data: {data}")

    # Send a welcome message to the client
    message = "ok"
    client_socket.send(message.encode())

def start_server():
    # Create a socket object
    server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

    # Bind the socket to a specific address and port
    host = '127.0.0.1'  # localhost
    port = 12345         # choose a port number

    server_socket.bind((host, port))

    # Listen for incoming connections
    server_socket.listen(5)
    print(f"Server listening on {host}:{port}")

    while True:
        # Accept a connection from a client
        client_socket, addr = server_socket.accept()

        # Handle the client's request
        handle_client(client_socket)

        # Close the client socket
        client_socket.close()

if __name__ == "__main__":
    start_server()