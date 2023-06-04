# Fotografia Computacional

## Trabalho 1

Nome: João Pedro Cosme da Silva / Cartão 0031472

## Introdução

O presente relatório tem como objetivo demonstrar os resultados obtidos na primeira tarefa para a cadeira de Fotografia Computacional, onde foi implementado uma pipeline de decodificação de imagens RAW englobando o processo de _demosaicking_, _white balance_ e _gamma encoding_. Para o desenvolvimento deste trabalho foi utilizada a linguagem Rust, utilizando pacotes para a leitura, gravação e manipulação de imagens RAW e imagens RGB.

## Demosaicking

Uma imagem RAW nos provém os pixeis de uma imagem tal qual foi capturado pela camera: como uma matriz de profundidade 1, onde cada um dos elementos só representa uma única cor. A câmera utiliza um *Color Filter Array* usando um padrão de Bayer, onde os pixeis são dispostos de forma que nas linhas pares temos pixeis na sequencia RG e nas linhas impares temos pixeis na sequencia GB.

Para a obtênção de uma imagem RGB, devemos interpolar os valores das cores faltantes para cada pixel. Ou seja, buscamos nos pixeis vizinhos os valores e interpolamos qual o provável valor daquela cor para determinado pixel. 

Quatro casos foram mapeados:

- Pixeis Vermelhos: Buscamos o valor de Verde nos pixeis horizontalmente vizinhos e o valor de Azul nos pixeis diagonalmente vizinhos.
- Pixeis Verdes - Linha Par: Buscamos os valores de Vermelho nos pixeis vizinhos horizontalmente e nos pixeis 3 passos em cada sentido e o valor de Azul nos pixeis vizinhos verticalmente e nos pixeis a passos em cada sentido.
- Pixeis Azuis: Buscamos o valor de Verde nos pixeis horizontalmente vizinhos e o valor de Vermelho nos pixeis diagonalmente vizinhos.
- Pixeis Verdes - Linha Impar: Buscamos os valores de Vermelho nos pixeis vizinhos verticalmente e nos pixeis 3 passos em cada sentido e o valor de Azul nos pixeis vizinhos horizontalmente e nos pixeis a passos em cada sentido.

Além disso, os valores obtidos pela câmera nativamente são de 12 bits, enquanto uma imagem RGB padrão utiliza 8 bits, a abordagem seguida foi de manter todos os pixeis normalizados em ponto flutuante entre 0 e 1 e escalados para estarem na faixa de 0 a 255 no momento em que as imagens forem salvas. Esta abordagem se deu por problemas no passo do gamma encoding onde transformar as imagens para 8bits prematuramente gerava uma imagem completamente escura como resultado.

O resultado obtido pelo demosaicking é o seguinte:

![Demosaick Image](./demosaick.png)

## White Balancing

Como visto na imagem anterior, o resultado obtido pela câmera se encontra com cores esverdeadas e não estão de acordo com o que esperamos da cena. Isso ocorre pois os sensores das cameras não possuem um ajuste de cor automática como nosso cérebro possui. Desta forma, precisamos ajustar as cores representadas na imagem através do processo de white balance para que se possa recuperar o aspecto esperado da cena.

Dois métodos foram utilizados aqui: Gray World e Scaling RGB.

### Gray World

Esse método se baseia na premissa de que uma imagem deve ter, como média, tons acinzentados. Desta forma, este método automático busca ajustar todos os valores de cor da imagem para um valor médio. O algoritmo se baseia em:

1. Encontrar as médias dos valores vermelho, verde e azul
1. Encontrar um fator alfa representado pela equação $Média Verde / Média Vermelho$
1. Encontrar um fator beta representado pela equação $Média Verde / Média Azul$
1. Para cada pixel da imagem, multiplicar o valor de vermelho por alfa e o valor de azul por beta 

O resultado obtido por esse método é o seguinte:

![gray world result](./white_balance_gray_world.png)

Podemos ver que de fato o resultado obtido é cinza e com poucas cores distintas.

### Scaling RGB

Ja neste processo, escolher um determinado valor da imagem e definimos o mesmo como o branco correto e em seguida encontramos fatores de cada cor em relação a 255 obter o fator de escala de cada cor.Em seguida, para cada pixel da imagem escalamos cada cor pelo fator pelo pixel encontrado. Para este caso, foi escolhido um dos icones na area de trabalho do monitor de valores RGB (139,231,134).

O resultado obtido foi o seguinte:
![scaling](./white_balance.png)

Podemos verificar que neste caso a imagem já possui cores muito mais vivas e diferenciadas em relação a abordagem anterior.

## Gamma Encoding

O que podemos perceber é que as imagens apresentadas anteriormente possuem uma aparência mais escura e desbotada em relação ao que esperamos da cena. Isto se dá pelo fato de que sensores de cameras reagem linearmente a luz, enquanto nossos cérebros reagem de maneira exponencial e reagimos mais a mudanças em tons mais escuros.

Dessa forma o método gamma encoding busca transformar esta variação linear em uma exponencial onde temos mais bits reservados por tons mais escuros. Para obter este resultado elevamos o valor de cada pixel a $\frac {1} {2.2}$.

O resultado obtido desta aplicação sobre a imagem sobre a imagem obtida do white balance Scaling RGB obtido no passo anterior:

![Gamma Encoding](gamma_encoded.png)

Com este resultado, podemos já perceber vários detalhes novos na imagem obtida, como artefatos na área de trabalho do notebook como exemplo.

## Conclusão

Com esta tarefa pude experimentar diretamente como lidar com imagens RAW, entendo sua complexidade e como cada parte desta pipeline reduzida funciona. Alguns dos desafios enfrentados na implementação foram a pesquisa de como cada um destes algoritmos deveria funcionar, bem como entender que existem diversos algoritmos para cada uma destas etapas. Os resultados obtidos foram inferiores ao que encontramos em cameras porém ainda sim foi possível verificar resultados de acordo com o esperado de cada etapa.