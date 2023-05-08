#include "wrapper.hpp"
#include "BairWitnessChecker_UTEST.hpp"
#include <cstdlib>
#include <cstring>
#include <iostream>
#include <sstream>
// #include <languages/Bair/BairWitness.hpp>
#include <string>

namespace RustyLibstark {
using Infrastructure::POW2;
using libstark::BairWitness;
using std::string;

int hello_from_c() { return 42; }

char *gen_hello() {

  char *string;
  string = (char *)malloc(sizeof(char) * 100);

  strcat(string, "hello");

  return string;
}

char *gen_witness() {
  const size_t vectorLen = 3;
  const short domainSizeIndicator = 3;
  const size_t domainSize = POW2(domainSizeIndicator);
  char *string;
  string = (char *)malloc(sizeof(char) * 10000);

  std::stringstream permutation_stream;
  std::stringstream assignment_stream;
  BairWitness witness = PCP_UTESTS::generate_valid_constraints().second;
  const BairWitness::permutation_t &permutation = witness.permutation();

  for (unsigned int i = 0; i < domainSize; i++) {
    size_t perm_img = permutation.getElementByIndex(i);
    BairWitness::color_t colors = witness.get_color(i);
    for (auto elem : colors) {
      std::string elem_str = elem.asString();
      assignment_stream << elem_str.c_str() << std::endl;
    }
    permutation_stream << perm_img << ',';
  }

  strcat(string, assignment_stream.str().c_str());
  strcat(string, "\n\n");
  strcat(string, permutation_stream.str().c_str());
  // std::cout << "HERE" << std::endl;
  // std::cout << string << std::endl;
  return string;
}
} // namespace RustyLibstark
