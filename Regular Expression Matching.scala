object Solution {
  def isMatch(str1: String, str2: String): Boolean = {
    str1.length >= 1 && str2.length >= 1 && str1.length <= 30 && str2.length <= 30 &&
    str1.matches(str2)
  }
}
