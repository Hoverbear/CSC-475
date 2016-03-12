GENRE_1=country
GENRE_2=disco
GENRE_3=classical

# Get the Genres
if [ ! -d genres ]; then
    curl http://opihi.cs.uvic.ca/sound/genres.tar.gz | tar xz
fi

# Build collections
if [ ! -f q1_${GENRE_1i}.mf ]; then
    mkcollection -c q1_${GENRE_1}.mf -l ${GENRE_1} genres/${GENRE_1}
fi

if [ ! -f q1_${GENRE_2}.mf ]; then
    mkcollection -c q1_${GENRE_2}.mf -l ${GENRE_2} genres/${GENRE_2}
fi

if [ ! -f q1_${GENRE_3}.mf ]; then
    mkcollection -c q1_${GENRE_3}.mf -l ${GENRE_3} genres/${GENRE_3}
fi

cat q1_${GENRE_1}.mf q1_${GENRE_2}.mf q1_${GENRE_3}.mf > q1.mf

bextract -sv q1.mf -w q1.arff

