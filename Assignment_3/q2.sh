if [ ! -d q2_data ]; then
    curl http://www.cs.cornell.edu/People/pabo/movie-review-data/review_polarity.tar.gz | tar xz
    mv txt_sentoken q2_data
fi


