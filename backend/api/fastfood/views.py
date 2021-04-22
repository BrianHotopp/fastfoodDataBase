from django.shortcuts import render

# Create your views here.
from fastfood.models import Menuentries, Restaurants
from rest_framework import viewsets
from rest_framework import permissions
from fastfood.serializers import MenuEntrySerializer, RestaurantSerializer


class ReadMenuEntryViewSet(viewsets.ReadOnlyModelViewSet):
    """
    api endpoint that allows menu entries to be viewed 
    """
    queryset = Menuentries.objects.all()
    serializer_class = MenuEntrySerializer
    permissions=[permissions.IsAuthenticated]

class ReadRestaurantViewSet(viewsets.ReadOnlyModelViewSet):
    """
    api endpoint that allows restaurants to be viewed 
    """
    queryset = Restaurants.objects.all()
    serializer_class = RestaurantSerializer
    permissions=[permissions.IsAuthenticated]


