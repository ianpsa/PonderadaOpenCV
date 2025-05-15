## Editor de Imagens do Ian

##### Ponderada de visualizador de imagens - Semana 03 
---


### Tecnologias Utilizadas
- [Rust](https://www.rust-lang.org/) - Linguagem de programação moderna focada em segurança e performance
- [Dioxus](https://dioxuslabs.com/) - Framework reativo para construção de interfaces gráficas multiplataforma
- [OpenCV](https://opencv.org/) - Biblioteca robusta para processamento e manipulação de imagens
- [RFD](https://github.com/PolyMeilex/rfd) - Biblioteca para diálogos nativos de seleção de arquivos

### Processamento de Imagens (image_process.rs)

O módulo image_process.rs implementa a lógica de processamento de imagens utilizando OpenCV, disponibilizando os seguintes filtros:

- **Grayscale (Escala de Cinza)**: Transforma a imagem colorida em tons de cinza
- **Invert (Inversão)**: Inverte todas as cores da imagem através de operação NOT
- **Contrast (Contraste)**: Intensifica o contraste da imagem multiplicando os valores dos pixels por 1.5
- **Blur (Desfoque)**: Suaviza a imagem aplicando um filtro gaussiano com matriz 5x5
- **Sharpen (Nitidez)**: Realça detalhes finos usando uma matriz de convolução personalizada:
  ```
  [0  -1   0]
  [-1  5  -1]
  [0  -1   0]
  ```
- **Edges (Detecção de Bordas)**: Identifica e destaca as bordas na imagem usando o algoritmo Canny
- **Reset**: Restaura a imagem ao seu estado original, removendo todos os filtros aplicados

### Interface do Usuário (ui.rs)

A interface gráfica foi desenvolvida com Dioxus, apresentando um layout intuitivo com:

- Visualização lado a lado da imagem original e processada
- Painel com botões para aplicação dos diferentes filtros
- Suporte para aplicação de múltiplos filtros em sequência
- Funcionalidade de reset para retornar à imagem original
- Sistema de seleção de arquivos

Os filtros podem ser combinados sequencialmente, permitindo efeitos complexos. A função reset chamada na UI retorna a imagem original.

## Como executar o projeto

1. Acesse a [página de releases](https://github.com/ianpsa/PonderadaOpenCV/releases/) do projeto no GitHub
2. Baixe o arquivo executável mais recente:
3. instale os pacotes necessários:
  ``` bash
  sudo apt update
  sudo apt install -y xdotool
  ou
  sudo pacman -S xdotool
  ```
4. Dê permissão de execução ao arquivo no Linux (não fechamos com macOS):
   ```bash
   chmod +x IanImageEditor
   ```
5. Execute o arquivo baixado:
   - Windows: Clique duas vezes no `.exe`
   - Linux: Execute via terminal:
     ```bash
     ./IanImageEditor
     ```
6. Agora com o programa aberto você pode editar as imagens.
