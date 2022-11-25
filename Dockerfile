FROM nixos/nix
COPY . /
RUN mkdir /out
ENTRYPOINT ["nix-shell", "--pure"]
