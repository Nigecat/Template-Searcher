FROM mcr.microsoft.com/windows/nanoserver:10.0.17134.1305

# Download autohotkey
RUN curl -o ahk.zip https://autohotkey.com/download/ahk.zip

# Uncompress autohotkey
RUN tar -x -f ahk.zip

RUN dir